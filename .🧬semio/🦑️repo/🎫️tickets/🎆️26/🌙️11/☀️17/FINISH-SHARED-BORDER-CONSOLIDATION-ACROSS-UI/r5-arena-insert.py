import re,sys
LINE='let _arena = semio_ui_contract_values::testkit::UiArenaFixture::new();'
for p in sys.argv[1:]:
    s=open(p).read()
    out=re.sub(r'(#\[test\]\n(?:#\[[^\n]*\]\n)*(?:async )?fn \w+\(\)\s*\{\n)(?!\s*let _arena)',lambda m:m.group(1)+'    '+LINE+'\n',s)
    open(p,'w').write(out)
    print(p,s.count('#[test]'),out.count('UiArenaFixture'))
