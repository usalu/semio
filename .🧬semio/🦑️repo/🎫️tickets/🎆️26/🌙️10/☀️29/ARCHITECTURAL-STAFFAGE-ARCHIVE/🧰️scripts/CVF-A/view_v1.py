import sys; from cva import *
aid, pose = sys.argv[1], sys.argv[2]
a = load_v1(aid); w = pose_world(a, pose); sh = a["geometry"]["elevation"]
bb = bbox_of(sh, w); print(aid, pose, "bbox", [round(v,3) for v in bb], "w", round(bb[2]-bb[0],3), "h", round(bb[3]-bb[1],3))
pad = 0.05
render(sh, w, {"body": 0.003, "detail": 0.0015, "texture": 0.001, "glazing": 0.001, "ground": 0.002, "mmToM": 0.05}, 700/ (bb[2]-bb[0]+2*pad) , (bb[0]-pad, -0.03, bb[2]+pad, bb[3]+pad), f"{OUT}/v1-{aid}-{pose}.png", grid=0.1)
