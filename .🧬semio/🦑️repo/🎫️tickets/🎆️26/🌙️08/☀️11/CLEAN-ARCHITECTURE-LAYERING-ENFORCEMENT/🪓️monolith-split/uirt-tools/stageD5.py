"""⏭️ Stage D5: narrow accessors the authority suites read the shadow and cursor through (testkit-gated), plus the detached retire used by the cursor suites."""
REPO="/Users/ueli/Documents/semio"
RT="🧰️framework/🔨️modules/🖱️ui/🧠️runtime"
p=f"{REPO}/{RT}/⏭️cursor/🦀️.rs"
t=open(p,encoding="utf-8").read()
CFG='#[cfg(any(test, feature = "testkit"))]'
def sub1(t,old,new):
    assert t.count(old)==1,(t.count(old),old[:80]); return t.replace(old,new)
t=sub1(t,"    /// 📦️ Moves every retained owner into a fresh shadow, leaving this one empty.",f'''    {CFG}
    pub fn retire_one_detached(&mut self) -> bool {{
        self.retire_one(&mut None::<()>, drop)
    }}

    {CFG}
    pub fn document(&self) -> Option<&ui_contract::UiDocumentLease> {{
        self.document.as_ref()
    }}

    /// 📦️ Moves every retained owner into a fresh shadow, leaving this one empty.''')
t=sub1(t,"    pub fn usage(&self) -> SurfaceReconcileUsage {\n        self.usage\n    }\n",f'''    pub fn usage(&self) -> SurfaceReconcileUsage {{
        self.usage
    }}

    {CFG}
    pub fn assembly_generation(&self) -> u64 {{
        self.assembly_generation
    }}

    {CFG}
    pub fn pending_root(&self) -> Option<&TreeNode> {{
        self.pending_root.as_ref()
    }}

    {CFG}
    pub fn held_node(&self) -> Option<&(Option<usize>, TreeNode)> {{
        self.held_node.as_ref()
    }}
''')
open(p,"w",encoding="utf-8").write(t)
print("ok")
