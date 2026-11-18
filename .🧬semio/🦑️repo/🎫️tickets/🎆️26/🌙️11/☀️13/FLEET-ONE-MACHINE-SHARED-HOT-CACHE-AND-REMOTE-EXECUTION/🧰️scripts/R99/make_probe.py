import sys,re,os
src,orig_dir,dst,extra=sys.argv[1:5]
text=open(src,encoding="utf8").read()
def fix(m):
    q,spec=m.group(1),m.group(2)
    if spec.startswith("."):
        return q+os.path.normpath(os.path.join(orig_dir,spec))+q
    return m.group(0)
text=re.sub(r'(["\'])(\.{1,2}/[^"\']+)\1',fix,text)
text+="\nexport {verifyProtectedArtifactInputFilesV1 as probeVerify};\n"
open(dst,"w",encoding="utf8").write(text)
print("probe",dst,len(text))
