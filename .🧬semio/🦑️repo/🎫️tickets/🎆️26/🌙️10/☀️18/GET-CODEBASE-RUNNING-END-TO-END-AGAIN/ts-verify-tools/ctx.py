import sys
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
f,l,c=sys.argv[1],int(sys.argv[2]),int(sys.argv[3])
w=int(sys.argv[4]) if len(sys.argv)>4 else 200
f=f.replace("L/",L,1) if f.startswith("L/") else "/Users/ueli/Documents/semio/"+f
s=open(f,encoding="utf-8").read().split("\n")[l-1]
print(f"[len {len(s)}] ..."+s[max(0,c-1-w):c-1]+"<<<HERE>>>"+s[c-1:c-1+w]+"...")
