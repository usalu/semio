import av, sys
from PIL import Image
for l in ("de","en"):
    p=f".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🗑️generated/lt/{l}/media/videos/🐍️/480p15/Beat3_GeraetePhiE.mp4"
    c=av.open(p); fr=[f.to_image() for f in c.decode(video=0)]
    n=len(fr); print(l,n)
    idx=[int(n*x) for x in (0.45,0.6,0.75,0.9)]
    w,h=fr[0].size; sheet=Image.new("RGB",(w*2,h*2))
    for k,i in enumerate(idx): sheet.paste(fr[i],((k%2)*w,(k//2)*h))
    sheet.save(f".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🗑️generated/lt/{l}.png")
