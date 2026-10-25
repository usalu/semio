import re,subprocess
files=subprocess.run(["rg","-l",r"PagedOneItemEdit<","-g","*.rs","✏️s"],capture_output=True,text=True).stdout.split("\n")
IMP="use semio_framework_value::retained_clone::{RetainedCloneRef, RetainedCloneStep};\nuse semio_framework_value::{ArtifactOwnedValueRetirementFactory, RetirementDemand};\nuse semio_framework_value_native_owned::native_decoding::NativeDecodeControl;\n"
for f in files:
    if not f: continue
    s=open(f,encoding="utf-8").read()
    if "fn advance_demands" in s: continue
    m=re.search(r"impl PagedOneItemEdit<(\w+), (\w+)> for (\w+) \{",s)
    if not m: print("skip",f[-50:]); continue
    S,M,E=m.groups()
    # struct field
    sm=re.search(r"(pub(?:\(super\))? struct "+E+r" \{)",s)
    s=s.replace(sm.group(1),sm.group(1)+"\n    closing: bool,",1)
    # imports after the PagedOneItem use line
    um=re.search(r"use semio_framework_plugin::plugin_app_close_prelude::store::\{[^}]*PagedOneItemEdit[^}]*\};\n",s)
    s=s.replace(um.group(0),um.group(0)+IMP,1)
    old=f"    fn advance(&mut self, post: &mut {S}, mutation: &{M}, grant: RetainedCloneGrant) -> Result<PagedOneItemEditStep<{M}>, ValueError> {{\n"
    assert old in s,f
    new=(f"    fn advance_demands(&self, _before: RetainedCloneRef<'_, {S}>, _mutation: RetainedCloneRef<'_, {M}>, _post: &{S}, _body: usize) -> Result<RetirementDemand, ValueError> {{\n        Ok(RetirementDemand::default())\n    }}\n\n"
         f"    fn advance(&mut self, _before: RetainedCloneRef<'_, {S}>, mutation: RetainedCloneRef<'_, {M}>, post: &mut {S}, _body: usize, grant: RetainedCloneGrant, _receiving: &mut NativeDecodeControl<'_>) -> Result<PagedOneItemEditStep<{M}>, ValueError> {{\n        let mutation = mutation.get();\n")
    s=s.replace(old,new,1)
    i=s.index(m.group(0)); j=s.index("\n}\n",i)
    life=(f"\n\n    fn begin_close(&mut self) {{\n        self.closing = true;\n    }}\n\n"
          f"    fn close_demands(&self, _mutations: &dyn ArtifactOwnedValueRetirementFactory<{M}>, _body: usize) -> Result<RetirementDemand, ValueError> {{\n        Ok(RetirementDemand::default())\n    }}\n\n"
          f"    fn close_step(&mut self, _mutations: &dyn ArtifactOwnedValueRetirementFactory<{M}>, _grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {{\n        self.closing = true;\n        Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))\n    }}\n\n"
          f"    fn terminal_is_empty(&self) -> bool {{\n        self.closing\n    }}")
    s=s[:j]+life+s[j:]
    open(f,"w",encoding="utf-8").write(s); print("ported",E,f[-60:])
