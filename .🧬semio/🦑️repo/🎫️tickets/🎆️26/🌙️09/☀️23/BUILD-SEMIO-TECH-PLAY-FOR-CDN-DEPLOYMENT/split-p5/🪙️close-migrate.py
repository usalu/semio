#!/usr/bin/env python3
"""🪙️ Migrates fem numerical jobs from the grant-shaped `InteractiveJob::close_step(grant)` protocol to the
original-context protocol (`close_step(cx)`, `OriginalTurnFailure`, no `Refused` variant), keeping each job's grant
ladder as `close_granted(grant) -> Result<InteractiveJobCloseStep, ValueError>`.

usage: 🪙️close-migrate.py <file.rs>...
"""
import re, sys

FRONTIER = re.compile(r"let frontier = match self\.close_frontier\(\) \{\s*Ok\(frontier\) => frontier,\s*Err\(error\) => return InteractiveJobCloseStep::Refused \{ kind: error\.kind, progress: error\.retained_progress\(\) \},\s*\};")

def block(s, start):
    i = s.index("{", start); d = 0; j = i
    while True:
        if s[j] == "{": d += 1
        elif s[j] == "}":
            d -= 1
            if d == 0: return i, j
        j += 1

def okify(body):
    body = FRONTIER.sub("let frontier = self.close_frontier()?;", body)
    body = re.sub(r"else \{ return (InteractiveJobCloseStep::Complete \{ progress: RetainedCloneProgress::default\(\) \}) \};", r"else { return Ok(\1) };", body)
    body = re.sub(r"return (InteractiveJobCloseStep::(?:Complete|Pending) \{ progress: [^;]*?\});", r"return Ok(\1);", body)
    body = re.sub(r"return step\.admit\(grant, self\.terminal_is_empty\(\)\);", "return step?.admit(grant, self.terminal_is_empty());", body)
    return body

def migrate(path):
    s = open(path, encoding="utf-8").read(); orig = s
    # restore cursors and other grant-shaped inherent closes
    for m in list(re.finditer(r"pub fn close_step\(&mut self, grant: RetainedCloneGrant\) -> InteractiveJobCloseStep \{", s))[::-1]:
        a, b = block(s, m.start())
        body = okify(s[a:b + 1])
        body = re.sub(r"InteractiveJob::close_step\((\w+), grant\)", r"\1.close_granted(grant)?", body)
        body = re.sub(r"(\n\s+)numerical_child_close\(step, terminal\)(\n\s*\}$)", r"\1Ok(numerical_child_close(step, terminal))\2", body)
        s = s[:m.start()] + "pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> " + body + s[b + 1:]
    # inner ladders
    for m in list(re.finditer(r"fn close_turn\(&mut self, grant: RetainedCloneGrant\) -> InteractiveJobCloseStep \{", s))[::-1]:
        a, b = block(s, m.start())
        body = s[a:b + 1]
        body = re.sub(r"(\n\s+)(numerical_ladder_turn\([^\n]*\))(\n\s*\}$)", r"\1Ok(\2)\3", body)
        s = s[:m.start()] + "fn close_turn(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> " + body + s[b + 1:]
    # trait impls
    for m in list(re.finditer(r"impl InteractiveJob for (\w+(?:<[^>]*>)?) \{", s))[::-1]:
        a, b = block(s, m.start())
        imp = s[a:b + 1]
        imp = imp.replace("-> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {", "-> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure> {")
        imp = re.sub(r"(\n\s+)self\.desk\.admit\(cx, run\)(\n\s*\})", r"\1Ok(self.desk.admit(cx, run)?)\2", imp)
        cm = re.search(r"\n    fn close_step\(&mut self, grant: RetainedCloneGrant\) -> InteractiveJobCloseStep \{", imp)
        if cm:
            ca, cb = block(imp, cm.start())
            old = imp[cm.start():cb + 1]
            granted_body = okify(imp[ca:cb + 1])
            granted_body = granted_body.replace("self.close_turn(grant).admit(grant, self.terminal_is_empty())", "self.close_turn(grant)?.admit(grant, self.terminal_is_empty())")
            granted_body = granted_body.replace("Some(step) => step,", "Some(step) => step?,")
            new = '''
    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
        if self.terminal_is_empty() {
            return Ok(InteractiveJobCloseStep::Complete { progress: RetainedCloneProgress::default() });
        }
        let demand = self.close_demand()?;
        numerical_close_original(cx, self, demand, Self::close_granted, Self::terminal_is_empty)
    }'''
            imp = imp.replace(old, new)
            name = m.group(1)
            granted = f'''

impl {name} {{
    /// 🎟️ One close turn under an explicit grant: the owners of a child job and the original-context `close_step`
    /// both admit it through the job's own quote.
    pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {{{granted_body.strip()[1:-1].rstrip()}
    }}
}}'''
            s = s[:a] + imp + granted + s[b + 1:]
        else:
            s = s[:a] + imp + s[b + 1:]
    if s != orig:
        s = re.sub(r"use semio_framework_job::\{(?!.*OriginalTurnFailure)", "use semio_framework_job::{OriginalTurnFailure, ", s, count=1)
        if "numerical_close_original" in s and "use semio_s_fem_sparse::{" in s and not re.search(r"use semio_s_fem_sparse::\{[^}]*numerical_close_original", s):
            s = s.replace("use semio_s_fem_sparse::{", "use semio_s_fem_sparse::{numerical_close_original, ", 1)
        open(path, "w", encoding="utf-8").write(s)
        print("migrated", path, "remaining Refused:", s.count("Refused"))

for p in sys.argv[1:]:
    migrate(p)
