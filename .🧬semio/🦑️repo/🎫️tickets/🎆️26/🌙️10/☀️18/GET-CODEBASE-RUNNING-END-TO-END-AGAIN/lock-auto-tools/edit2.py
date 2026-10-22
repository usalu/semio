import sys
M="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧭️host/📦️packages/🦀️rust/Cargo.toml"
LINE='semio-framework-tool-run = { path = "../../../../../../../../../../../🔨️modules/⏯️tool-run/📦️packages/🦀️rust" }\n'
s=open(M).read()
if sys.argv[1]=="add":
    assert LINE not in s
    open(M,"w").write(s.replace("[dependencies]\n","[dependencies]\n"+LINE,1))
elif sys.argv[1]=="remove":
    assert LINE in s
    open(M,"w").write(s.replace(LINE,"",1))
print(sys.argv[1],"ok")
