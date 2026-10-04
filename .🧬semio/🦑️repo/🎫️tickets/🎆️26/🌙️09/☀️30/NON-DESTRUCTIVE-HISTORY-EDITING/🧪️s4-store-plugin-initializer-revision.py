#!/usr/bin/env python3
"""🔏️ S4-STORE (audit F3, design §3.7): every plugin-owned retained store initializer names the content revision a whole load
names — the canonical initial digest (`store::artifact_initial_digest`) and the store's canonical, effective-forward edit records
(`push_applied_edit` / `push_redo_edit`) — so the per-operation hash phases, the domain digests and the digest fields they fed are
deleted. Exact, anchored rewrites per initializer family; refuses on any missing anchor; idempotent (`--check` lists pending).
Usage: python3 🧪️s4-store-plugin-initializer-revision.py [--check] [Name …]"""
import re
import sys

ROOT = "✏️s/🔌️plugins/"
FILES = {
    "GisMap": "🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs",
    "Jack": "🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime/🦀️.rs",
    "Generation2d": "🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs",
    "Generation3d": "🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs",
    "Process3d": "🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs",
    "Drawing": "🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️.rs",
    "Raster": "🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs",
}
OBSERVE_TESTS = {
    "Generation2d": "🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️retained-authority-laws/🦀️.rs",
    "Generation3d": "🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️retained-authority-laws/🦀️.rs",
    "Process3d": "🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️retained-laws/🦀️.rs",
}


def top_level_span(text, start):
    """The span of the top-level item starting at `start` (its doc comment and attributes included) up to its closing `}`."""
    head = start
    while True:
        previous = text.rfind("\n", 0, head - 1) + 1
        line = text[previous:head - 1] if head > 0 else ""
        if head > 0 and (line.startswith("///") or line.startswith("#[")):
            head = previous
            continue
        break
    end = re.compile(r"^\}\n", re.M).search(text, start)
    return head, end.end()


def inplace(edit, prefix):
    p = edit.phase
    observe = re.compile(r"^fn " + prefix + r"_observe_\w+\(", re.M)
    while True:
        match = observe.search(edit.text)
        if not match:
            break
        head, end = top_level_span(edit.text, match.start())
        edit.text = edit.text[:head] + edit.text[end:]
    stepper = re.search(r"^( +)fn step\(&mut self, source: &\w+, digest: &mut store::ArtifactStoreInitializationDigest(?:, cx: &mut semio_framework_job::StepContext<'_>)?\)[^\n]*\{\n", edit.text, re.M)
    if not stepper:
        edit.fail("copy stepper missing")
    close = re.compile("^" + stepper.group(1) + r"\}\n", re.M).search(edit.text, stepper.end())
    body = edit.text[stepper.end():close.start()]
    body = re.sub(r"^ +digest\.observe\([^\n]*\);\n", "", body, flags=re.M)
    body = re.sub(r"^ +" + prefix + r"_observe_\w+\(digest, [^\n]*\);\n", "", body, flags=re.M)
    if re.search(r"\bdigest\b", body):
        edit.fail("copy stepper still reads its digest")
    signature = edit.text[stepper.start():stepper.end()].replace(", digest: &mut store::ArtifactStoreInitializationDigest", "")
    edit.text = edit.text[:stepper.start()] + signature + body + edit.text[close.start():]
    edit.sub(r"\.step\(source, self\.initial_digest\.as_mut\(\)\.expect\(\"[^\"]+\"\)(, cx)?\)", r".step(source\1)", 1)
    edit.sub(r"^( +)let (digest|initial_digest) = self\.initial_digest\.take\(\)\.expect\(\"[^\"]+\"\)\.finish\(\);\n", r"\1let \2 = store::artifact_initial_digest(&initial);\n", 1)
    edit.sub(r"^ +let mut digest = store::ArtifactStoreInitializationDigest::new\(b\"[^\"]+\"\);\n(?: +digest\.observe\([^\n]+\);\n)+ +\*self\.edit_digest = Some\(digest\);\n", "", 2)
    edit.rep(f"self.phase = {p}::HashRedoForward {{ position, edit: scan, mutation: 0 }};", f"self.phase = {p}::CommitRedo {{ position, edit: scan }};")
    for variant in ("HashInverse", "HashRedoForward", "HashRedoInverse"):
        edit.cut_arm(variant)
    edit.sub(r"^ +" + prefix + r"_observe_mutation\(self\.edit_digest\.as_mut\(\)\.expect\(\"[^\"]+\"\), operation\);\n", "", 1)
    edit.rep(f"self.phase = {p}::HashInverse {{ position, edit, mutation: 0 }};", f"self.phase = {p}::CommitApplied {{ position, edit }};")
    edit.sub(r"^ +let id = " + prefix + r"_copy_string\(&entry\.id\)\.unwrap_or_default\(\);\n", "", 1 if prefix == "process3d" else 2)
    edit.sub(r"^ +let digest = self\.edit_digest\.take\(\)\.expect\(\"[^\"]+\"\)\.finish\(\);\n", "", 2)
    edit.rep("push_applied(id, digest)", "push_applied_edit(entry)")
    if prefix == "process3d":
        edit.sub(r"^( +)let id = (self\.envelope\.as_ref\(\)\.and_then\(\|envelope\| envelope\.vcs\.edits\.get\(edit\)\))\.and_then\(\|entry\| process3d_copy_string\(&entry\.id\)\.ok\(\)\)\.unwrap_or_default\(\);\n", r'\1let entry = \2.expect("Process3d redo edit retained");\n', 1)
    edit.rep("push_redo(id, digest)", "push_redo_edit(entry)")
    for name in ("initial_digest", "edit_digest"):
        edit.sub(r"^ +" + name + r": std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,\n", "", 1)
    edit.sub(r"^ +initial_digest: std::mem::ManuallyDrop::new\(Some\(store::ArtifactStoreInitializationDigest::new\(b\"[^\"]+\"\)\)\),\n", "", 1)
    edit.sub(r"^ +edit_digest: std::mem::ManuallyDrop::new\(None\),\n", "", 1)
    edit.sub(r"^ +&& self\.initial_digest\.is_none\(\)\n", "", 1)
    edit.sub(r"^ +&& self\.edit_digest\.is_none\(\)\n", "", 1)
    edit.sub(r"^ +\*self\.initial_digest = None;\n", "", 3)
    edit.sub(r"^ +\*self\.edit_digest = None;\n", "", 3)


def drop_observe_test(name, prefix):
    path = ROOT + OBSERVE_TESTS[name]
    text = open(path, encoding="utf-8").read()
    match = re.search(r"^ *" + prefix + r"_observe_mutation\(&mut left", text, re.M)
    if not match:
        return
    fn = text.rfind("\nfn ", 0, match.start())
    fn = max(fn, text.rfind("\nasync fn ", 0, match.start()))
    head, end = top_level_span(text, fn + 1)
    open(path, "w", encoding="utf-8").write(text[:head] + text[end:])


class Edit:
    def __init__(self, name, text):
        self.name, self.text, self.phase = name, text, f"{name}StoreInitializationPhase"

    def fail(self, what):
        sys.exit(f"{self.name}: {what}")

    def rep(self, old, new, count=1):
        if self.text.count(old) != count:
            self.fail(f"anchor count {self.text.count(old)}: {old[:110]!r}")
        self.text = self.text.replace(old, new)

    def sub(self, pattern, new, count):
        found = len(re.findall(pattern, self.text, re.M))
        if count is not None and found != count:
            self.fail(f"pattern count {found}: {pattern[:110]!r}")
        self.text = re.sub(pattern, new, self.text, flags=re.M)

    def cut_arm(self, variant):
        match = re.search(r"^( +)" + self.phase + "::" + variant + r"\b[^\n]*=> \{\n", self.text, re.M)
        if not match:
            self.fail(f"arm {variant} missing")
        end = re.compile("^" + match.group(1) + r"\}\n", re.M).search(self.text, match.end())
        self.text = self.text[: match.start()] + self.text[end.end():]

    def common(self):
        p = self.phase
        self.sub(r"^ +HashInverse \{ position: usize, edit: usize, mutation: usize \},\n", "", 1)
        self.sub(r"^ +HashRedoForward \{ position: usize, edit: usize, mutation: usize \},\n", "", 1)
        self.sub(r"^ +HashRedoInverse \{ position: usize, edit: usize, mutation: usize \},\n", "", 1)
        self.sub(r"^ +initial_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,\n", "", 1)
        self.sub(r"^ +edit_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,\n", "", 1)
        self.sub(r"^ +initial_digest: std::mem::ManuallyDrop::new\(Some\(store::ArtifactStoreInitializationDigest::new\(b\"[^\"]+\"\)\)\),\n", "", 1)
        self.sub(r"^ +edit_digest: std::mem::ManuallyDrop::new\(None\),\n", "", 1)
        self.sub(r"^ +&& self\.initial_digest\.is_none\(\)\n", "", 1)
        self.sub(r"^ +&& self\.edit_digest\.is_none\(\)\n", "", 1)
        self.sub(r"^ +\*self\.initial_digest = None;\n", "", 3)
        self.sub(r"^ +\*self\.edit_digest = None;\n", "", 3)
        self.sub(r"^( +)let initial_digest = self\.initial_digest\.take\(\)\.expect\(\"[^\"]+\"\)\.finish\(\);\n", r"\1let initial_digest = store::artifact_initial_digest(&initial);\n", 1)
        self.sub(
            r"^( +)if edit\.id == id \{\n(?: +let (?:id|sequence_number|started_at) = edit\.[a-z_]+(?:\.clone\(\))?;\n)* +let mut digest = store::ArtifactStoreInitializationDigest::new\(b\"[^\"]+\"\);\n(?: +digest\.observe\([^\n]+\);\n)+ +\*self\.edit_digest = Some\(digest\);\n( +)self\.phase = " + p + r"::ApplyForward \{ position, edit: scan, mutation: 0 \};\n",
            r"\1if edit.id == id {\n\2self.phase = " + p + "::ApplyForward { position, edit: scan, mutation: 0 };\n",
            1,
        )
        self.sub(
            r"^( +)if edit\.id == id \{\n(?: +let (?:id|sequence_number|started_at) = edit\.[a-z_]+(?:\.clone\(\))?;\n)* +let mut digest = store::ArtifactStoreInitializationDigest::new\(b\"[^\"]+\"\);\n(?: +digest\.observe\([^\n]+\);\n)+ +\*self\.edit_digest = Some\(digest\);\n( +)self\.phase = " + p + r"::HashRedoForward \{ position, edit: scan, mutation: 0 \};\n",
            r"\1if edit.id == id {\n\2self.phase = " + p + "::CommitRedo { position, edit: scan };\n",
            1,
        )
        self.sub(
            r"^ +let Some\(operation\) = entry\.forwards\.get\(mutation\) else \{\n +self\.phase = " + p + r"::HashInverse \{ position, edit, mutation: 0 \};\n +return semio_framework_job::StepOutcome::Yield;\n +\};\n +let encoded = match operation\.encode_op\(\) \{\n[^\n]*\n +_ => \{\n[^\n]*\n[^\n]*\n +\}\n +\};\n +self\.edit_digest\.as_mut\(\)\.expect\(\"[^\"]+\"\)\.observe\(&encoded\);\n",
            "",
            1,
        )
        self.sub(r"^( +)let entry = self\.envelope\.as_ref\(\)\.and_then\(\|envelope\| envelope\.vcs\.edits\.get\(edit\)\)\.expect\(\"[^\"]+\"\);\n(?=( +)let envelope = self\.envelope\.as_ref\(\)\.expect\(\"[^\"]+ while its forwards fold\"\);)", "", 1)
        self.rep(f"Ok(store::ArtifactStoreInitializationForward::Exhausted) => self.phase = {p}::HashInverse {{ position, edit, mutation: 0 }},", f"Ok(store::ArtifactStoreInitializationForward::Exhausted) => self.phase = {p}::CommitApplied {{ position, edit }},")
        for variant in ("HashInverse", "HashRedoForward", "HashRedoInverse"):
            self.cut_arm(variant)
        self.sub(
            r"^( +)let id = entry\.id\.clone\(\);\n( +let actor = entry\.actor\.clone\(\);\n) +let digest = self\.edit_digest\.take\(\)\.expect\(\"[^\"]+\"\)\.finish\(\);\n( +let runtime = self\.runtime\.as_mut\(\)\.expect\(\"[^\"]+\"\);\n +if let Err\(error\) = runtime\.)push_applied\(id, digest\)",
            r"\2\3push_applied_edit(entry)",
            1,
        )
        self.sub(
            r"^( +)let id = (self\.envelope\.as_ref\(\)\.and_then\(\|envelope\| envelope\.vcs\.edits\.get\(edit\)\)\.expect\(\"[^\"]+\"\))\.id\.clone\(\);\n +let digest = self\.edit_digest\.take\(\)\.expect\(\"[^\"]+\"\)\.finish\(\);\n( +if let Err\(error\) = self\.runtime\.as_mut\(\)\.expect\(\"[^\"]+\"\)\.)push_redo\(id, digest\)",
            r"\1let entry = \2;\n\3push_redo_edit(entry)",
            1,
        )

    def clone_stepper(self):
        self.sub(r"(fn step\(&mut self, source: &\w+), digest: &mut store::ArtifactStoreInitializationDigest(, cx: &mut semio_framework_job::StepContext<'_>\))", r"\1\2", 1)
        self.sub(r"clone\.step\(source, self\.initial_digest\.as_mut\(\)\.expect\(\"[^\"]+\"\), cx\)", "clone.step(source, cx)", 1)


def gismap(edit):
    edit.common()
    edit.clone_stepper()
    edit.rep("                    digest.observe(&encoded);\n", "")
    edit.rep("        digest.observe(observed);\n", "")


def jack(edit):
    edit.common()
    edit.clone_stepper()
    edit.sub(r"^        let phase = self\.phase;\n        let active = self\.active\.is_some\(\);\n(        let step = self\.advance\(source, JACK_OWNED_FIELD_BYTES\)\?;\n)        if !active \{\n            match phase \{\n(?:                [^\n]*\n)+?            \}\n        \}\n", r"\1", 1)


def generation2d(edit):
    inplace(edit, "generation2d")
    drop_observe_test("Generation2d", "generation2d")


def generation3d(edit):
    inplace(edit, "generation3d")
    drop_observe_test("Generation3d", "generation3d")


def process3d(edit):
    inplace(edit, "process3d")
    drop_observe_test("Process3d", "process3d")


def stepped_common(edit, label):
    """The shared part of the stepped-candidate initializers (drawing, raster): no edit digest, no per-op hash phases, the
    mutation digest prelude of `ApplyForward` gone, the commit phases carrying their edit for the canonical record."""
    p = edit.phase
    for variant in ("HashInverse", "HashRedoForward", "HashRedoInverse"):
        edit.sub(r"^ +" + variant + r" \{ position: usize, edit: usize, mutation: usize \},\n", "", 1)
        edit.cut_arm(variant)
    edit.sub(r"^ +mutation_digest: std::mem::ManuallyDrop<Option<\w+MutationDigestAuthority>>,\n", "", 1)
    edit.sub(r"^ +mutation_digest: std::mem::ManuallyDrop::new\(None\),\n", "", 1)
    edit.sub(r"^ +&& self\.mutation_digest\.is_none\(\)\n", "", 1)
    for name in ("initial_digest", "edit_digest"):
        edit.sub(r"^ +" + name + r": std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,\n", "", 1)
        edit.sub(r"^ +&& self\." + name + r"\.is_none\(\)\n", "", 1)
        edit.sub(r"^ +\*self\." + name + r" = None;\n", "", 3)
    edit.sub(r"^ +initial_digest: std::mem::ManuallyDrop::new\(Some\(store::ArtifactStoreInitializationDigest::new\(b\"[^\"]+\"\)\)\),\n", "", 1)
    edit.sub(r"^ +edit_digest: std::mem::ManuallyDrop::new\(None\),\n", "", 1)
    edit.sub(r"^ +drop\(self\.mutation_digest\.take\(\)\);\n(?= +(?:\*self\.|self\.terminal_handoff|drop\(self\.mutation_candidate|Ok\(|Some\(candidate|\}))", "", None)
    edit.sub(r"^( +)if let Some\(digest\) = self\.mutation_digest\.as_mut\(\) \{\n(?:.*\n)*?\1\}\n", "", 1)
    edit.sub(r"^( +)let (?:initial_digest|digest) = self\.initial_digest\.take\(\)\.expect\(\"[^\"]+\"\)\.finish\(\);\n", r"\1let initial_digest = store::artifact_initial_digest(&initial);\n", 1)
    edit.sub(r"^ +let mut digest = store::ArtifactStoreInitializationDigest::new\(b\"[^\"]+\.edit\"\);\n(?: +digest\.observe\([^\n]+\);\n)+ +\*self\.edit_digest = Some\(digest\);\n", "", 2)
    forward = re.search(r"^( +)" + p + r"::ApplyForward \{ position, edit, mutation \} => \{\n", edit.text, re.M)
    start = edit.text.index("                if self.mutation_digest.is_none() {\n", forward.end())
    end = edit.text.index("                drop(self.mutation_digest.take());\n", start) + len("                drop(self.mutation_digest.take());\n")
    edit.text = edit.text[:start] + edit.text[end:]
    edit.sub(r"^( +)let entry = self\.envelope\.as_ref\(\)\.and_then\(\|envelope\| envelope\.vcs\.edits\.get\(edit\)\)\.expect\(\"" + label + r" applied edit remains retained\"\);\n +let Some\(operation\) = entry\.forwards\.get\(mutation\) else \{\n", r'\1if self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).is_none_or(|entry| mutation >= entry.forwards.len()) {\n', 1)
    edit.rep("                    return semio_framework_job::StepOutcome::Yield;\n                };\n                let withdrawn = {", "                    return semio_framework_job::StepOutcome::Yield;\n                }\n                let withdrawn = {")
    if "mutation_digest" in edit.text and label == "Raster":
        edit.fail("raster mutation digest references left")


def drawing(edit):
    p = edit.phase
    edit.sub(r"^ +HashInitialSchema,\n +HashInitialId,\n", "", 1)
    edit.cut_arm("HashInitialSchema")
    edit.cut_arm("HashInitialId")
    edit.rep(f"store::ArtifactStoreInitializationEditAdmission::Complete => self.phase = {p}::HashInitialSchema,", f"store::ArtifactStoreInitializationEditAdmission::Complete => self.phase = {p}::CloneInitialHeader,")
    stepped_common(edit, "Drawing")
    edit.rep(f"self.phase = {p}::HashInverse {{ position, edit, mutation: 0 }};", f"self.phase = {p}::PrepareApplied {{ position, edit, field: 0 }};")
    edit.rep(f"self.phase = {p}::HashRedoForward {{ position, edit: scan, mutation: 0 }};", f"self.phase = {p}::PrepareRedo {{ position, edit: scan }};")
    edit.rep("    CommitApplied { position: usize },\n", "    CommitApplied { position: usize, edit: usize },\n")
    edit.rep("    CommitRedo { position: usize },\n", "    CommitRedo { position: usize, edit: usize },\n")
    edit.rep(f"self.phase = {p}::CommitApplied {{ position }};", f"self.phase = {p}::CommitApplied {{ position, edit }};")
    edit.rep(f"self.phase = {p}::CommitRedo {{ position }};", f"self.phase = {p}::CommitRedo {{ position, edit }};")
    edit.rep(f"""            {p}::CommitApplied {{ position }} => {{
                let id = self.prepared_history_id.take().expect("Drawing applied id was retained in its own preparation grant");
                let actor = self.prepared_actor.take();
                let digest = self.edit_digest.take().expect("Drawing applied edit digest remains retained").finish();
                let runtime = self.runtime.as_mut().expect("Drawing runtime remains retained");
                if let Err(error) = runtime.push_applied(id, digest) {{""", f"""            {p}::CommitApplied {{ position, edit }} => {{
                let id = self.prepared_history_id.take().expect("Drawing applied id was retained in its own preparation grant");
                let actor = self.prepared_actor.take();
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Drawing applied edit remains retained");
                let runtime = self.runtime.as_mut().expect("Drawing runtime remains retained");
                if let Err(error) = runtime.push_applied_admitted(id, entry) {{""")
    edit.rep(f"""            {p}::CommitRedo {{ position }} => {{
                let id = self.prepared_history_id.take().expect("Drawing redo id was retained in its own preparation grant");
                let digest = self.edit_digest.take().expect("Drawing redo digest remains retained").finish();
                if let Err(error) = self.runtime.as_mut().expect("Drawing runtime remains retained").push_redo(id, digest) {{""", f"""            {p}::CommitRedo {{ position, edit }} => {{
                let id = self.prepared_history_id.take().expect("Drawing redo id was retained in its own preparation grant");
                let entry = self.envelope.as_ref().and_then(|envelope| envelope.vcs.edits.get(edit)).expect("Drawing redo edit remains retained");
                if let Err(error) = self.runtime.as_mut().expect("Drawing runtime remains retained").push_redo_admitted(id, entry) {{""")


def raster(edit):
    p = edit.phase
    start = edit.text.index("struct RasterMutationDigestAuthority {\n")
    drop_impl = edit.text.index("impl Drop for RasterMutationDigestAuthority {\n", start)
    end = edit.text.index("\n}\n", drop_impl) + 3
    edit.text = edit.text[:start] + edit.text[end:].lstrip("\n")
    layer = re.search(r"^    fn step\(&mut self, source_root: &RasterLayerNode, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>\)[^\n]*\{\n", edit.text, re.M)
    if not layer:
        edit.fail("layer clone stepper missing")
    close = re.compile(r"^    \}\n", re.M).search(edit.text, layer.end())
    body = edit.text[layer.end():close.start()]
    for old, new in (
        ("                let locked=crate::standards::v1::subsets::any::schema::layer_locked(source);\n", ""),
        ("                if frame.phase==0 {digest.observe(&[u8::from(locked)]);}\n", ""),
        ("digest.observe(key.as_bytes());}", "}"),
    ):
        if body.count(old) != 1:
            edit.fail(f"layer clone anchor {old[:60]!r}")
        body = body.replace(old, new)
    body = re.sub(r"^ +digest\.observe\([^\n]*\);\n", "", body, flags=re.M)
    if re.search(r"\bdigest\b", body):
        edit.fail("layer clone still reads its digest")
    edit.text = edit.text[:layer.start()] + edit.text[layer.start():layer.end()].replace(", digest: &mut store::ArtifactStoreInitializationDigest", "") + body + edit.text[close.start():]
    snapshot = re.search(r"^    fn step\(&mut self, source: &RasterSnapshot, digest: &mut store::ArtifactStoreInitializationDigest, cx: &mut semio_framework_job::StepContext<'_>\)[^\n]*\{\n", edit.text, re.M)
    close = re.compile(r"^    \}\n", re.M).search(edit.text, snapshot.end())
    body = edit.text[snapshot.end():close.start()]
    for old, new in (
        ("        let observed: &[u8] = match self.phase {\n", "        match self.phase {\n"),
        ("                target.schema = raster_clone_owned_string(&source.schema)?;\n                source.schema.as_bytes()\n", "                target.schema = raster_clone_owned_string(&source.schema)?;\n"),
        ("                target.id = raster_clone_owned_string(&source.id)?;\n                source.id.as_bytes()\n", "                target.id = raster_clone_owned_string(&source.id)?;\n"),
        ("                    target.title = Some(raster_clone_owned_string(title)?);\n                    title.as_bytes()\n", "                    target.title = Some(raster_clone_owned_string(title)?);\n"),
        ("                    if !raster_reserve_unit(cx) {\n                        return Ok(false);\n                    }\n                    &[]\n                }\n", "                    if !raster_reserve_unit(cx) {\n                        return Ok(false);\n                    }\n                }\n"),
        ("                self.index = 0;\n                &[]\n            }\n", "                self.index = 0;\n            }\n"),
        (", digest, cx)? {", ", cx)? {"),
        ("        };\n        digest.observe(observed);\n", "        }\n"),
    ):
        if body.count(old) != 1:
            edit.fail(f"snapshot clone anchor {old[:70]!r}")
        body = body.replace(old, new)
    body = re.sub(r"^ +digest\.observe\([^\n]*\);\n", "", body, flags=re.M)
    if re.search(r"\bdigest\b", body):
        edit.fail("snapshot clone still reads its digest")
    edit.text = edit.text[:snapshot.start()] + edit.text[snapshot.start():snapshot.end()].replace(", digest: &mut store::ArtifactStoreInitializationDigest", "") + body + edit.text[close.start():]
    edit.sub(r"clone\.step\(source, self\.initial_digest\.as_mut\(\)\.expect\(\"[^\"]+\"\), cx\)", "clone.step(source, cx)", 1)
    stepped_common(edit, "Raster")
    edit.rep(f"self.phase = {p}::HashInverse {{ position, edit, mutation: 0 }};", f"self.phase = {p}::CommitApplied {{ position, edit }};")
    edit.rep(f"self.phase = {p}::HashRedoForward {{ position, edit: scan, mutation: 0 }};", f"self.phase = {p}::CommitRedo {{ position, edit: scan }};")
    edit.sub(r"^( +)let id = entry\.id\.clone\(\);\n( +let actor = entry\.actor\.clone\(\);\n) +let digest = self\.edit_digest\.take\(\)\.expect\(\"[^\"]+\"\)\.finish\(\);\n( +let runtime = self\.runtime\.as_mut\(\)\.expect\(\"[^\"]+\"\);\n +if let Err\(error\) = runtime\.)push_applied\(id, digest\)", r"\2\3push_applied_edit(entry)", 1)
    edit.sub(r"^( +)let id = (self\.envelope\.as_ref\(\)\.and_then\(\|envelope\| envelope\.vcs\.edits\.get\(edit\)\)\.expect\(\"[^\"]+\"\))\.id\.clone\(\);\n +let digest = self\.edit_digest\.take\(\)\.expect\(\"[^\"]+\"\)\.finish\(\);\n( +if let Err\(error\) = self\.runtime\.as_mut\(\)\.expect\(\"[^\"]+\"\)\.)push_redo\(id, digest\)", r"\1let entry = \2;\n\3push_redo_edit(entry)", 1)


RULES = {"GisMap": gismap, "Jack": jack, "Generation2d": generation2d, "Generation3d": generation3d, "Process3d": process3d, "Drawing": drawing, "Raster": raster}


def main():
    check = "--check" in sys.argv
    names = [name for name in sys.argv[1:] if not name.startswith("--")] or list(FILES)
    pending = []
    for name in names:
        path = ROOT + FILES[name]
        text = open(path, encoding="utf-8").read()
        if "edit_digest" not in text and "initial_digest: std::mem::ManuallyDrop" not in text:
            continue
        pending.append(name)
        if check:
            continue
        edit = Edit(name, text)
        RULES[name](edit)
        if "edit_digest" in edit.text or "self.initial_digest" in edit.text:
            edit.fail("digest references left")
        open(path, "w", encoding="utf-8").write(edit.text)
    print(("pending: " if check else "rewritten: ") + (", ".join(pending) or "none"))
    sys.exit(1 if check and pending else 0)


if __name__ == "__main__":
    main()
