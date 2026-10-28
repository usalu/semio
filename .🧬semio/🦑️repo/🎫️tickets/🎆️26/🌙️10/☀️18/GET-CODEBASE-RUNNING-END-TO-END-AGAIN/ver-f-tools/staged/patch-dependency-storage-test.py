"""usage: patch-dependency-storage-test.py <bootstrap/dependencies/storage/tests file> — the extracted producer-planning body reads inspectJavascriptDependencyInputPlansReadV1; the harness binds it from the storage owner like its sibling."""
import sys
p=sys.argv[1];s=open(p).read()
a='inspectJavascriptDependencyInputsReadV1:storage.inspectJavascriptDependencyInputsReadV1,'
assert s.count(a)==1,s.count(a)
open(p,'w').write(s.replace(a,a+'inspectJavascriptDependencyInputPlansReadV1:storage.inspectJavascriptDependencyInputPlansReadV1,'))
