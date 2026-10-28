import re, sys, json
# Streams Rust test files; reports files that anchor at CARGO_MANIFEST_DIR / current_dir AND write with std::fs below a path that is not a temp dir.
files=[l.rstrip("\n") for l in open(sys.argv[1],encoding="utf8")]
anchor=re.compile(r'CARGO_MANIFEST_DIR|current_dir\(\)')
write=re.compile(r'fs::(write|create_dir_all|create_dir|remove_dir_all|remove_file|rename|copy)\(|File::create\(|OpenOptions::new\(\)')
temp=re.compile(r'temp_dir|tempfile|TempDir|tempdir|env::temp_dir|SEMIO_TEST_ARTIFACT_DIR|std::env::var\("TMPDIR"|unique_dir|scratch')
out=[]
for f in files:
    try: t=open(f,encoding="utf8",errors="replace").read()
    except OSError: continue
    a=len(anchor.findall(t)); w=len(write.findall(t))
    if a and w:
        out.append({"f":f,"anchor":a,"writes":w,"temp":bool(temp.search(t))})
json.dump(out,sys.stdout,ensure_ascii=False)
print(file=sys.stderr)
print(len(out),sum(1 for o in out if not o["temp"]),file=sys.stderr)
