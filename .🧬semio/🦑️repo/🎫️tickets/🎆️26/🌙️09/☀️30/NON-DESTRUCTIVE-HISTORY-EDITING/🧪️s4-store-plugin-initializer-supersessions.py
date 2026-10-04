#!/usr/bin/env python3
"""🪞️ S4-STORE (W1G-2): the 8 plugin-owned retained store initializers fold the EFFECTIVE forwards (design §3.1). Each gets the
shared stepped supersession fold after seeding (`ArtifactStoreInitializationRuntime::fold_supersession_step`) and applies every
forward through its effective input — `fold_forward` (keep-and-record, as every other fold site) for the generic diff/apply
initializers, `effective_forward` for the in-place and stepped ones. Idempotent: an already migrated file is left alone.
Usage: python3 🧪️s4-store-plugin-initializer-supersessions.py [--check]"""
import re
import sys

ROOT = "✏️s/🔌️plugins/"
PLUGINS = {
    "Writer": ("✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs", "generic", "WRITER_ENVELOPE_FIELD_BYTES", "WriterSnapshotRetirementFactory", "writer-store", "Writer"),
    "GisMap": ("🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs", "generic", "GIS_MAP_OWNED_FIELD_BYTES", "GisMapSnapshotRetirementFactory", "gis-map-store", "GIS"),
    "Jack": ("🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime/🦀️.rs", "generic", "JACK_OWNED_FIELD_BYTES", "JackSnapshotRetirementFactory", "jack-store", "Jack"),
    "Generation2d": ("🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs", "inplace", "generation2d_apply_initialization_mutation", None, "generation2d-store", "P2"),
    "Generation3d": ("🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs", "inplace", "generation3d_apply_initialization_mutation", None, "generation3d-store", "P3"),
    "Process3d": ("🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs", "inplace", "process3d_apply_retained_mutation", None, "process3d-store", "Process3d"),
    "Drawing": ("🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs", "stepped", None, None, "drawing-store", "Drawing"),
    "Raster": ("🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs", "stepped", None, None, "raster-store", "Raster"),
}


def fail(name, what):
    sys.exit(f"{name}: {what}")


def arm_span(text, phase, variant):
    """The span of the match arm `<phase>::<variant> … => {` up to the next arm of the same phase enum at that indentation."""
    match = re.search(r"^( +)" + re.escape(phase) + "::" + variant + r"\b[^\n]*=> \{\n", text, re.M)
    if not match:
        return None
    indent = match.group(1)
    closing = re.compile("^" + indent + r"\}\n", re.M).search(text, match.end())
    return match.start(), closing.end(), indent


def migrate(name, text):
    path, kind, apply, factory, code, label = PLUGINS[name]
    phase = f"{name}StoreInitializationPhase"
    if f"{phase}::FoldSupersessions" in text:
        return text, False
    variant = re.search(r"^( +)SeedHistory \{ edit: usize, lane: u8, index: usize \},\n", text, re.M)
    if not variant:
        fail(name, "SeedHistory variant missing")
    text = text[: variant.end()] + f"{variant.group(1)}FoldSupersessions {{ transition: usize }},\n" + text[variant.end():]
    seed = arm_span(text, phase, "SeedHistory")
    if not seed:
        fail(name, "SeedHistory arm missing")
    body = text[seed[0]:seed[1]]
    target = f"self.phase = {phase}::FindApplied {{ position: 0 }};"
    if body.count(target) != 1:
        fail(name, f"SeedHistory end transition count {body.count(target)}")
    body = body.replace(target, f"self.phase = {phase}::FoldSupersessions {{ transition: 0 }};")
    indent = seed[2]
    inner = indent + "    "
    valued = body.rstrip().rstrip("}").rstrip().endswith("semio_framework_job::StepOutcome::Yield")
    tail = f"{inner}cx.consume_fuel(1);\n{inner}semio_framework_job::StepOutcome::Yield\n" if valued else ""
    fold_arm = (
        f"{indent}{phase}::FoldSupersessions {{ transition }} => {{\n"
        f'{inner}let envelope = self.envelope.as_ref().expect("{label} envelope remains retained while its supersessions fold");\n'
        f'{inner}match self.runtime.as_mut().expect("{label} runtime remains retained while its supersessions fold").fold_supersession_step(envelope, transition) {{\n'
        f"{inner}    Ok(true) => self.phase = {phase}::FoldSupersessions {{ transition: transition + 1 }},\n"
        f"{inner}    Ok(false) => self.phase = {phase}::FindApplied {{ position: 0 }},\n"
        f"{inner}    Err(error) => {{\n"
        f"{inner}        self.fault = Some(error.into_bytes());\n"
        f"{inner}        self.phase = {phase}::RetireFault;\n"
        f"{inner}    }}\n"
        f"{inner}}}\n"
        f"{tail}"
        f"{indent}}}\n"
    )
    text = text[: seed[0]] + body + fold_arm + text[seed[1]:]
    forward = arm_span(text, phase, "ApplyForward")
    if not forward:
        fail(name, "ApplyForward arm missing")
    body = text[forward[0]:forward[1]]
    if kind == "generic":
        start = body.find("                let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut)")
        if start < 0:
            fail(name, "generic apply anchor missing")
        new_tail = (
            f'{inner}let envelope = self.envelope.as_ref().expect("{label} envelope remains retained while its forwards fold");\n'
            f'{inner}let entry = envelope.vcs.edits.get(edit).expect("{label} applied edit remains retained");\n'
            f'{inner}match self.runtime.as_mut().expect("{label} runtime remains retained while its forwards fold").fold_forward(entry, mutation, &envelope.schema, {apply}) {{\n'
            f"{inner}    Ok(store::ArtifactStoreInitializationForward::Folded {{ displaced, fuel }}) => {{\n"
            f"{inner}        if let Some(previous) = displaced {{\n"
            f"{inner}            *self.active = Some(store::ArtifactOwnedValueRetirementFactory::retire_owned(&{factory}, previous));\n"
            f"{inner}        }}\n"
            f"{inner}        self.phase = {phase}::ApplyForward {{ position, edit, mutation: mutation + 1 }};\n"
            f"{inner}        cx.consume_fuel(fuel as u64);\n"
            f"{inner}    }}\n"
            f"{inner}    Ok(store::ArtifactStoreInitializationForward::Exhausted) => self.phase = {phase}::HashInverse {{ position, edit, mutation: 0 }},\n"
            f'{inner}    Err(_) => self.fail(b"{code}.initializer-forward-encoding"),\n'
            f"{inner}}}\n"
            f"{inner}semio_framework_job::StepOutcome::Yield\n"
            f"{indent}}}\n"
        )
        body = body[:start] + new_tail
    elif kind == "inplace":
        old = re.search(r"( +)let current = self\.runtime\.as_mut\(\)\.and_then\(store::ArtifactStoreInitializationRuntime::current_mut\)\.expect\(\"[^\"]*\"\);\n +match " + re.escape(apply) + r"\(current, operation\) \{\n +Ok\(retired\) => \{\n +\*self\.active = retired;\n +self\.phase = " + re.escape(phase) + r"::ApplyForward \{ position, edit, mutation: mutation \+ 1 \};\n +\}\n +Err\(code\) => self\.fail\(code\.as_bytes\(\)\),\n +\}\n", body)
        if not old:
            fail(name, "in-place apply anchor missing")
        new = (
            f'{inner}let envelope = self.envelope.as_ref().expect("{label} envelope remains retained while its forwards fold");\n'
            f'{inner}let entry = envelope.vcs.edits.get(edit).expect("{label} applied edit remains retained");\n'
            f'{inner}let effective = self.runtime.as_ref().and_then(|runtime| runtime.effective_forward(entry, mutation, &envelope.schema)).expect("{label} applied forward remains retained");\n'
            f'{inner}let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect("{label} runtime current retained");\n'
            f"{inner}let applied = effective.operation().map(|operation| {apply}(current, operation));\n"
            f"{inner}drop(effective);\n"
            f"{inner}match applied {{\n"
            f"{inner}    Some(Ok(retired)) => {{\n"
            f"{inner}        *self.active = retired;\n"
            f"{inner}        self.phase = {phase}::ApplyForward {{ position, edit, mutation: mutation + 1 }};\n"
            f"{inner}    }}\n"
            f"{inner}    None => self.phase = {phase}::ApplyForward {{ position, edit, mutation: mutation + 1 }},\n"
            f"{inner}    Some(Err(code)) => self.fail(code.as_bytes()),\n"
            f"{inner}}}\n"
        )
        body = body[: old.start()] + new + body[old.end():]
    else:
        marker = "                drop(self.mutation_digest.take());\n"
        if body.count(marker) != 1:
            fail(name, "stepped digest anchor missing")
        skip = (
            f"{inner}let withdrawn = {{\n"
            f'{inner}    let envelope = self.envelope.as_ref().expect("{label} envelope remains retained while its forwards fold");\n'
            f"{inner}    envelope.vcs.edits.get(edit).and_then(|entry| self.runtime.as_ref().and_then(|runtime| runtime.effective_forward(entry, mutation, &envelope.schema))).is_none_or(|effective| effective.operation().is_none())\n"
            f"{inner}}};\n"
            f"{inner}if withdrawn {{\n"
            f"{inner}    self.phase = {phase}::ApplyForward {{ position, edit, mutation: mutation + 1 }};\n"
            f"{inner}    cx.consume_fuel(1);\n"
            f"{inner}    return semio_framework_job::StepOutcome::Yield;\n"
            f"{inner}}}\n"
        )
        body = body.replace(marker, marker + skip)
        step = re.search(r"( +)let current = self\.runtime\.as_mut\(\)\.and_then\(store::ArtifactStoreInitializationRuntime::current_mut\)\.expect\((?P<current>\"[^\"]*\")\);\n +let candidate_complete = match (?P<receiver>self\.mutation_candidate\.as_mut\(\)\.expect\(\"[^\"]*\"\))\.step\(current, operation, cx\) \{\n +Ok\((?P<binding>\w+)\) => (?P=binding),\n +Err\((?P<error>\w+)\) => \{\n +self\.fail\((?P=error)\.as_bytes\(\)\);\n +return semio_framework_job::StepOutcome::Yield;\n +\}\n +\};\n", body)
        if not step:
            fail(name, "stepped candidate step anchor missing")
        binding, error = step.group("binding"), step.group("error")
        stepped = (
            f'{inner}let envelope = self.envelope.as_ref().expect("{label} envelope remains retained while its forwards fold");\n'
            f'{inner}let effective = envelope.vcs.edits.get(edit).and_then(|entry| self.runtime.as_ref().and_then(|runtime| runtime.effective_forward(entry, mutation, &envelope.schema))).expect("{label} applied forward remains retained");\n'
            f"{inner}let current = self.runtime.as_mut().and_then(store::ArtifactStoreInitializationRuntime::current_mut).expect({step.group('current')});\n"
            f'{inner}let stepped = {step.group("receiver")}.step(current, effective.operation().expect("{label} effective forward was checked"), cx);\n'
            f"{inner}drop(effective);\n"
            f"{inner}let candidate_complete = match stepped {{\n"
            f"{inner}    Ok({binding}) => {binding},\n"
            f"{inner}    Err({error}) => {{\n"
            f"{inner}        self.fail({error}.as_bytes());\n"
            f"{inner}        return semio_framework_job::StepOutcome::Yield;\n"
            f"{inner}    }}\n"
            f"{inner}}};\n"
        )
        body = body[: step.start()] + stepped + body[step.end():]
    text = text[: forward[0]] + body + text[forward[1]:]
    return text, True


def main():
    check = "--check" in sys.argv
    pending = []
    for name in PLUGINS:
        path = ROOT + PLUGINS[name][0]
        text = open(path, encoding="utf-8").read()
        migrated, changed = migrate(name, text)
        if changed:
            pending.append(name)
            if not check:
                open(path, "w", encoding="utf-8").write(migrated)
    print(("pending: " if check else "migrated: ") + (", ".join(pending) or "none"))
    sys.exit(1 if check and pending else 0)


if __name__ == "__main__":
    main()
