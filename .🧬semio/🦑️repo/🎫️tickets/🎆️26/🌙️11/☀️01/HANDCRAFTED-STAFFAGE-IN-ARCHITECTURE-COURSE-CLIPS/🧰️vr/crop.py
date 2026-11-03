import sys, glob, av
root, lang, scene, idx, x0, y0, x1, y1 = sys.argv[1:9]
c = av.open(glob.glob(f"{root}/media-{lang}/videos/**/{scene}.mp4", recursive=True)[0])
for i, f in enumerate(c.decode(video=0)):
    if i == int(idx):
        im = f.to_image().crop((int(x0), int(y0), int(x1), int(y1)))
        im = im.resize((im.width * 3, im.height * 3))
        im.save(f"{root}/{lang}-{scene}-crop.png")
        break
