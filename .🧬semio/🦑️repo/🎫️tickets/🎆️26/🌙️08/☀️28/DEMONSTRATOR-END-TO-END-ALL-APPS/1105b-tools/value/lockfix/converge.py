import re,sys
p=sys.argv[1];t=open(p).read()
parts=re.split(r'(?m)^(?=\[\[package\]\]\n)',t)
head,pk=parts[0],parts[1:]
drop={('js-sys','0.3.107'):'0.3.106',('web-sys','0.3.107'):'0.3.106',('wasm-bindgen','0.2.130'):'0.2.129',('wasm-bindgen-macro','0.2.130'):'0.2.129',('wasm-bindgen-macro-support','0.2.130'):'0.2.129',('wasm-bindgen-shared','0.2.130'):'0.2.129'}
out=[];n=0
for b in pk:
    nm=re.search(r'name = "([^"]+)"',b).group(1);v=re.search(r'version = "([^"]+)"',b).group(1)
    if (nm,v) in drop:n+=1;continue
    out.append(b)
res=head+''.join(out)
for (nm,v),to in drop.items():
    res=res.replace('"%s %s"'%(nm,v),'"%s %s"'%(nm,to))
print("dropped",n)
open(p,'w').write(res)
