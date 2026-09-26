#!/usr/bin/env python3
"""🏎️ H10 post-publish patch set Q1 (ticket 26/09/23 session 12): the owned interpreter stops paying per instruction for
bookkeeping that belongs to the step, the call or nothing at all.

Profile (`sample` of the scratch probe running `codec.pack-schema-hash` on B2's note component, 7–11 M instr/s):
`CoreInstance::step` + `execute_machine` ≈ 80 % of samples, the rest malloc/free/memmove. Per instruction the tree:
  * moves the whole `Machine` out of `self.machine` and back (`execute_instruction`),
  * clones two `Arc`s (function body + control-bound map) to look at the current function,
  * clones a `ControlFrame` (two `Vec<ValueType>`) on every `end`, `else` and taken branch,
  * copies block/branch/return results into a fresh `Vec` and back (`take_results`),
  * clones the callee's `FunctionDecl` (locals `Vec`) and `FunctionType` on every call.
The patch keeps every observable exactly: one fuel unit per executed instruction, the same traps with the same texts,
the same `Machine`/`Frame`/`ControlFrame` shapes (so checkpoints are byte-identical and `CHECKPOINT_VERSION` stays 1):
  * `step` takes the machine out once per step and holds one `Arc<CoreModule>` for the step;
  * `execute_machine` borrows the body and bounds from that module;
  * `else`/`end`/branches read the control frame in place and move results within the operand stack
    (`keep_results`, same checks in the same order as `take_results`);
  * calls and returns borrow the callee's declaration and type from one module `Arc`;
  * LEB128 immediates that fit one byte (almost all of them) decode without the general loop.
Laws: the interpreter unit suite (unchanged), plus `--scratch` here: every codec row of a real trusted catalog (B2),
old vs new interpreter on the same component: identical fuel, identical output bytes, and the output equal to the
catalog's own trust record (the hash the publish pipeline pinned) for every row the guest answers (a codec-only
package such as stdio answers `Err` for kinds it leaves to the hub's native codec: `GUEST-UNOWNED`, still required
identical) — plus the measured speedup.

usage (every mode takes --root <repository copy> to patch a copy instead of the tree): q1-interpreter-speed.py            dry run
       q1-interpreter-speed.py --apply    write the tree (post-publish window only)
       q1-interpreter-speed.py --scratch [package…]   patched COPY under wp-h10/target/q1-scratch vs the tree's interpreter
"""
import json, os, pathlib, subprocess, sys, time

ROOT = pathlib.Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else pathlib.Path("/Users/ueli/Documents/semio")
INTERPRETER = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🦀️.rs"
H10 = pathlib.Path("/Users/ueli/Documents/semio/.tmp-ticket/wp-h10")
CATALOG = ROOT / ".🧬semio/🌐hub/w2-catalog-b2/trusted-catalog/generations/f485bf7e725255cd5b70787796e477a249578d788a49960564d7613dce46f2e1"

HUNKS = [
    ("step",
     """        if let Some(call) = self.pending_host_call().cloned() {
            return CoreStepOutcome::HostCall { fuel_used: used, call };
        }
        while used < fuel {
            if control.cancelled {
                self.machine = None;
                return CoreStepOutcome::Cancelled { fuel_used: used };
            }
            match self.execute_instruction() {
                Ok(InstructionProgress::Continue) => used += 1,
                Ok(InstructionProgress::Host(call)) => {
                    used += 1;
                    return CoreStepOutcome::HostCall { fuel_used: used, call };
                }
                Ok(InstructionProgress::Complete(values)) => {
                    used += 1;
                    self.machine = None;
                    return CoreStepOutcome::Complete { fuel_used: used, values };
                }
                Err(error) => {
                    used += 1;
                    let error = self.diagnose_fault(error);
                    self.machine = None;
                    return CoreStepOutcome::Fault { fuel_used: used, error };
                }
            }
        }
        CoreStepOutcome::Yield { fuel_used: used }
    }""",
     """        if let Some(call) = self.pending_host_call().cloned() {
            return CoreStepOutcome::HostCall { fuel_used: used, call };
        }
        let Some(mut machine) = self.machine.take() else { return CoreStepOutcome::Fault { fuel_used: used, error: CoreError::State("no invocation is active".into()) } };
        let module = Arc::clone(&self.module);
        while used < fuel {
            if control.cancelled {
                return CoreStepOutcome::Cancelled { fuel_used: used };
            }
            match self.execute_machine(&module, &mut machine) {
                Ok(InstructionProgress::Continue) => used += 1,
                Ok(InstructionProgress::Host(call)) => {
                    used += 1;
                    self.machine = Some(machine);
                    return CoreStepOutcome::HostCall { fuel_used: used, call };
                }
                Ok(InstructionProgress::Complete(values)) => {
                    used += 1;
                    return CoreStepOutcome::Complete { fuel_used: used, values };
                }
                Err(error) => {
                    used += 1;
                    self.machine = Some(machine);
                    let error = self.diagnose_fault(error);
                    self.machine = None;
                    return CoreStepOutcome::Fault { fuel_used: used, error };
                }
            }
        }
        self.machine = Some(machine);
        CoreStepOutcome::Yield { fuel_used: used }
    }"""),
    ("execute_instruction + body borrow",
     """    fn execute_instruction(&mut self) -> Result<InstructionProgress, CoreError> {
        let mut machine = self.machine.take().ok_or_else(|| CoreError::State("no invocation is active".into()))?;
        let result = self.execute_machine(&mut machine);
        if !matches!(result, Ok(InstructionProgress::Complete(_))) {
            self.machine = Some(machine);
        }
        result
    }

    fn execute_machine(&mut self, machine: &mut Machine) -> Result<InstructionProgress, CoreError> {
        let frame_index = machine.frames.len().checked_sub(1).ok_or_else(|| CoreError::State("active invocation has no call frame".into()))?;
        let function = machine.frames[frame_index].function;
        let (body, controls) = match self.module.functions.get(function as usize) {
            Some(FunctionDecl::Defined { body, controls, .. }) => (body.clone(), controls.clone()),
            _ => return Err(CoreError::State("call frame points at an import".into())),
        };
        let instruction_pc = machine.frames[frame_index].pc;
        let mut decoder = Decoder::at(&body, instruction_pc);""",
     """    fn execute_machine(&mut self, code: &CoreModule, machine: &mut Machine) -> Result<InstructionProgress, CoreError> {
        let frame_index = machine.frames.len().checked_sub(1).ok_or_else(|| CoreError::State("active invocation has no call frame".into()))?;
        let function = machine.frames[frame_index].function;
        let (body, controls) = match code.functions.get(function as usize) {
            Some(FunctionDecl::Defined { body, controls, .. }) => (&body[..], &**controls),
            _ => return Err(CoreError::State("call frame points at an import".into())),
        };
        let instruction_pc = machine.frames[frame_index].pc;
        let mut decoder = Decoder::at(body, instruction_pc);"""),
    ("else",
     """            0x05 => {
                let control = machine.frames[frame_index].controls.last().cloned().ok_or_else(|| CoreError::Trap("else has no control frame".into()))?;
                if control.kind != ControlKind::If {
                    return Err(CoreError::Trap("else is not inside an if".into()));
                }
                close_control(machine, &control)?;
                machine.frames[frame_index].controls.pop();
                decoder.position = control.end_pc + 1;
            }""",
     """            0x05 => {
                let kind = machine.frames[frame_index].controls.last().map(|control| control.kind).ok_or_else(|| CoreError::Trap("else has no control frame".into()))?;
                if kind != ControlKind::If {
                    return Err(CoreError::Trap("else is not inside an if".into()));
                }
                let control = machine.frames[frame_index].controls.pop().expect("checked");
                keep_results(machine, control.stack_height, &control.result_types)?;
                decoder.position = control.end_pc + 1;
            }"""),
    ("end",
     """            0x0b => {
                let control = machine.frames[frame_index].controls.last().cloned().ok_or_else(|| CoreError::Trap("end has no control frame".into()))?;
                if control.kind == ControlKind::Function {
                    return self.return_frame(machine);
                }
                close_control(machine, &control)?;
                machine.frames[frame_index].controls.pop();
            }""",
     """            0x0b => {
                let kind = machine.frames[frame_index].controls.last().map(|control| control.kind).ok_or_else(|| CoreError::Trap("end has no control frame".into()))?;
                if kind == ControlKind::Function {
                    return self.return_frame(machine);
                }
                let control = machine.frames[frame_index].controls.pop().expect("checked");
                keep_results(machine, control.stack_height, &control.result_types)?;
            }"""),
    ("return + branch",
     """    fn return_frame(&mut self, machine: &mut Machine) -> Result<InstructionProgress, CoreError> {
        let frame = machine.frames.pop().ok_or_else(|| CoreError::Trap("return has no frame".into()))?;
        let results = self.module.function_type(frame.function)?.results.clone();
        let values = take_results(machine, frame.stack_base, &results)?;
        if machine.frames.is_empty() {
            return Ok(InstructionProgress::Complete(values));
        }
        machine.values.extend(values);
        Ok(InstructionProgress::Continue)
    }

    fn branch(&mut self, machine: &mut Machine, depth: u32) -> Result<InstructionProgress, CoreError> {
        let frame_index = machine.frames.len().checked_sub(1).ok_or_else(|| CoreError::Trap("branch has no frame".into()))?;
        let control_count = machine.frames[frame_index].controls.len();
        let target_index = control_count.checked_sub(depth as usize + 1).ok_or_else(|| CoreError::Trap(format!("branch depth {depth} is out of bounds")))?;
        let target = machine.frames[frame_index].controls[target_index].clone();
        let values = take_results(machine, target.stack_height, &target.branch_types)?;
        machine.values.extend(values);
        match target.kind {""",
     """    fn return_frame(&mut self, machine: &mut Machine) -> Result<InstructionProgress, CoreError> {
        let frame = machine.frames.pop().ok_or_else(|| CoreError::Trap("return has no frame".into()))?;
        let module = Arc::clone(&self.module);
        keep_results(machine, frame.stack_base, &module.function_type(frame.function)?.results)?;
        if machine.frames.is_empty() {
            return Ok(InstructionProgress::Complete(machine.values.split_off(frame.stack_base)));
        }
        Ok(InstructionProgress::Continue)
    }

    fn branch(&mut self, machine: &mut Machine, depth: u32) -> Result<InstructionProgress, CoreError> {
        let frame_index = machine.frames.len().checked_sub(1).ok_or_else(|| CoreError::Trap("branch has no frame".into()))?;
        let control_count = machine.frames[frame_index].controls.len();
        let target_index = control_count.checked_sub(depth as usize + 1).ok_or_else(|| CoreError::Trap(format!("branch depth {depth} is out of bounds")))?;
        let control = &mut machine.frames[frame_index].controls[target_index];
        let (kind, stack_height, start_pc, end_pc) = (control.kind, control.stack_height, control.start_pc, control.end_pc);
        let branch_types = std::mem::take(&mut control.branch_types);
        let kept = keep_results(machine, stack_height, &branch_types);
        machine.frames[frame_index].controls[target_index].branch_types = branch_types;
        kept?;
        let target = BranchTargetV1 { kind, start_pc, end_pc };
        match target.kind {"""),
    ("results helpers",
     """fn close_control(machine: &mut Machine, control: &ControlFrame) -> Result<(), CoreError> {
    let values = take_results(machine, control.stack_height, &control.result_types)?;
    machine.values.extend(values);
    Ok(())
}
""",
     """/// 🎯️ What a taken branch needs of its target once the results are in place.
struct BranchTargetV1 {
    kind: ControlKind,
    start_pc: usize,
    end_pc: usize,
}

/// 📥️ Leaves exactly `types` on top of the operand stack at `stack_height`: the same checks, in the same
/// order and with the same traps, as [`take_results`] followed by re-pushing its values — without the
/// intermediate copy, which every block end, branch and return used to allocate.
fn keep_results(machine: &mut Machine, stack_height: usize, types: &[ValueType]) -> Result<(), CoreError> {
    if stack_height > machine.values.len() {
        return Err(CoreError::Trap("control stack height exceeds operand stack".into()));
    }
    let start = machine.values.len().checked_sub(types.len()).ok_or_else(|| CoreError::Trap("control results underflow the operand stack".into()))?;
    if start < stack_height {
        return Err(CoreError::Trap("control results overlap the outer operand stack".into()));
    }
    check_values(&machine.values[start..], types)?;
    machine.values.copy_within(start.., stack_height);
    machine.values.truncate(stack_height + types.len());
    Ok(())
}
"""),
    ("unsigned LEB fast path",
     """    fn u64(&mut self) -> Result<u64, CoreError> {
        let mut value = 0u64;""",
     """    fn u64(&mut self) -> Result<u64, CoreError> {
        if let Some(&byte) = self.bytes.get(self.position).filter(|byte| **byte & 0x80 == 0) {
            self.position += 1;
            return Ok(u64::from(byte));
        }
        let mut value = 0u64;"""),
    ("signed LEB fast path",
     """    fn signed(&mut self, bits: u32) -> Result<i64, CoreError> {
        let mut value = 0i64;""",
     """    fn signed(&mut self, bits: u32) -> Result<i64, CoreError> {
        if let Some(&byte) = self.bytes.get(self.position).filter(|byte| **byte & 0x80 == 0 && bits > 7) {
            self.position += 1;
            let value = i64::from(byte & 0x7f);
            return Ok(if byte & 0x40 != 0 { value | (!0i64 << 7) } else { value });
        }
        let mut value = 0i64;"""),
    ("enter_function_on",
     """        let declaration = self.module.functions.get(function as usize).ok_or_else(|| CoreError::Trap(format!("function {function} is out of bounds")))?.clone();
        let function_type = self.module.function_type(function)?.clone();
        check_values(&arguments, &function_type.parameters)?;
        match declaration {
            FunctionDecl::Import { module, name, .. } => {
                let call = HostCall { id: self.next_host_call, module, name, arguments, results: function_type.results };""",
     """        let code = Arc::clone(&self.module);
        let declaration = code.functions.get(function as usize).ok_or_else(|| CoreError::Trap(format!("function {function} is out of bounds")))?;
        let function_type = code.function_type(function)?;
        check_values(&arguments, &function_type.parameters)?;
        match declaration {
            FunctionDecl::Import { module, name, .. } => {
                let call = HostCall { id: self.next_host_call, module: module.clone(), name: name.clone(), arguments, results: function_type.results.clone() };"""),
    ("enter_function_on defined",
     """            FunctionDecl::Defined { locals, body, .. } => {
                let stack_base = machine.values.len();
                let mut all_locals = arguments;
                all_locals.extend(locals.into_iter().map(ValueType::zero));""",
     """            FunctionDecl::Defined { locals, body, .. } => {
                let stack_base = machine.values.len();
                let mut all_locals = arguments;
                all_locals.extend(locals.iter().copied().map(ValueType::zero));"""),
    ("enter_function_on frame",
     """                    controls: vec![ControlFrame { kind: ControlKind::Function, start_pc: 0, end_pc, stack_height: stack_base, branch_types: function_type.results.clone(), result_types: function_type.results }],""",
     """                    controls: vec![ControlFrame { kind: ControlKind::Function, start_pc: 0, end_pc, stack_height: stack_base, branch_types: function_type.results.clone(), result_types: function_type.results.clone() }],"""),
]

def patched():
    text = INTERPRETER.read_text(encoding="utf-8")
    for label, old, new in HUNKS:
        count = text.count(old)
        if count != 1:
            raise SystemExit(f"hunk '{label}' found {count}× — re-derive it")
        text = text.replace(old, new)
    return text

text = patched()
print(f"hunks OK: {len(HUNKS)} in {INTERPRETER.name}")
if "--apply" in sys.argv:
    INTERPRETER.write_text(text, encoding="utf-8")
    print("applied — now: nice -n 15 cargo test -p semio-framework-plugin-host --lib -- interpreter; hub + MCP laws; wasm32 not affected (host-only module)")
elif "--scratch" in sys.argv:
    scratch = H10 / "target/q1-scratch"
    scratch.mkdir(parents=True, exist_ok=True)
    (scratch / "interpreter.rs").write_text(text.replace('#[path = "🧪️tests/🔬️unit/🦀️.rs"]', f'#[path = "{INTERPRETER.parent}/🧪️tests/🔬️unit/🦀️.rs"]'), encoding="utf-8")
    probe = (H10 / "q1-interpreter/main.rs").read_text(encoding="utf-8").replace('#[path = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🦀️.rs"]', f'#[path = "{scratch}/interpreter.rs"]')
    (scratch / "main.rs").write_text(probe, encoding="utf-8")
    (scratch / "Cargo.toml").write_text('[package]\nname = "h10-owned-probe-q1"\nversion = "0.0.0"\nedition = "2021"\npublish = false\n\n[workspace]\n\n[[bin]]\nname = "h10-owned-probe-q1"\npath = "main.rs"\n\n[profile.release]\nopt-level = 3\ndebug = false\n', encoding="utf-8")
    env = dict(os.environ, CARGO_TARGET_DIR=str(H10 / "target"), CARGO_INCREMENTAL="0")
    for crate in (H10 / "q1-interpreter", scratch):
        result = subprocess.run(["nice", "-n", "15", "cargo", "build", "--release"], cwd=crate, env=env)
        if result.returncode != 0:
            raise SystemExit(result.returncode)
    old, new = H10 / "target/release/h10-owned-probe", H10 / "target/release/h10-owned-probe-q1"
    bundle = json.loads((CATALOG / "trusted-catalog.json").read_text(encoding="utf-8"))
    wanted = [argument for argument in sys.argv[2:] if not argument.startswith("--")]
    failures = 0
    for package in bundle["packages"]:
        if wanted and package["packageId"].split(":")[-1] not in wanted:
            continue
        component = CATALOG / package["component"]["path"]
        for codec in package["nativeCodecs"]:
            rows = {}
            for label, binary in (("old", old), ("new", new)):
                started = time.time()
                output = subprocess.run([str(binary), "codec", str(component), "pack-schema-hash", codec["artifactSchema"], "", "1"], capture_output=True, text=True).stdout
                row = next((line for line in output.splitlines() if line.startswith("round=0")), "")
                fields = dict(part.split("=", 1) for part in row.split() if "=" in part and not part.startswith("head="))
                rows[label] = (fields.get("fuel"), fields.get("outputFnv"), float(fields.get("ms", "nan")), fields.get("okHex", ""))
            identical = rows["old"][0] == rows["new"][0] and rows["old"][1] == rows["new"][1] and rows["old"][0] is not None
            answered = rows["new"][3] != ""
            trusted = rows["new"][3] == codec["packSchemaHash"]
            verdict = "TRUSTED" if trusted else "GUEST-UNOWNED" if not answered else "UNTRUSTED"
            failures += 0 if identical and verdict != "UNTRUSTED" else 1
            print(f"{'SAME' if identical else 'DIFF'} {verdict} {package['packageId']} {codec['artifactSchema']} fuel={rows['new'][0]} old={rows['old'][2]:.0f}ms new={rows['new'][2]:.0f}ms speedup={rows['old'][2] / max(rows['new'][2], 1e-9):.2f}x", flush=True)
    print(f"DONE failures={failures}")
    raise SystemExit(1 if failures else 0)
else:
    print("dry run: nothing written")
