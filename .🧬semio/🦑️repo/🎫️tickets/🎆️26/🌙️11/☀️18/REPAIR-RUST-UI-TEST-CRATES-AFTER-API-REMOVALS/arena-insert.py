"""🏟️ Opens one serialized UI arena scope as the first statement of each named law (args: file fn [fn ...])."""
import re,sys
path,fns=sys.argv[1],sys.argv[2:]
s=open(path).read()
for fn in fns:
    m=re.search(r'fn '+re.escape(fn)+r'\s*\(\s*\)\s*(?:->[^{]*)?\{',s)
    assert m,(path,fn)
    body=s[m.end():m.end()+200]
    if "UiArenaFixture::new()" in body:continue
    indent=re.match(r'\n?([ \t]*)',body).group(1) or "    "
    sep="\n"+indent if body.startswith("\n") else ""
    s=s[:m.end()]+(sep if sep else "")+"let _arena = semio_ui_contract_values::testkit::UiArenaFixture::new();"+("" if sep else "")+s[m.end():]
open(path,"w").write(s)
