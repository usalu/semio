import os,re,sys
R="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/"
S=R+"🏅️standards/"
def w(f,s):
    t=f+".f46tmp";open(t,'w',encoding='utf8').write(s);os.replace(t,f)
M="semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::"
for f in [S+"🔟ac1024/🪆️subsets/✳️any/✏️editor/🦀️.rs",S+"4️⃣ac1018/🪆️subsets/✳️any/✏️editor/🦀️.rs",S+"🔟ac1024/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]:
    s=open(f,encoding='utf8').read()
    n=s.count(M+"export::")+s.count(M+"import::")
    s=s.replace(M+"export::","semio_s_artifact_stdio_dwg_write::standards::v_ac1024::subsets::any::io::export::").replace(M+"import::","semio_s_artifact_stdio_dwg_read::standards::v_ac1024::subsets::any::io::import::")
    w(f,s);print(f[-60:],n)
for f in [S+"🔟ac1024/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml",S+"4️⃣ac1018/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml"]:
    s=open(f,encoding='utf8').read()
    a="semio-s-artifact-stdio-dwg-model = { workspace = true }\n"
    assert s.count(a)==1 and "dwg-write" not in s
    s=s.replace(a,a+"semio-s-artifact-stdio-dwg-read = { workspace = true }\nsemio-s-artifact-stdio-dwg-write = { workspace = true }\n",1)
    w(f,s);print("manifest",f[-70:])
