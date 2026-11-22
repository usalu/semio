"""⛏️ Excavator 2D at 1:200: the track frame was drawn down to the chain (y 0.125) and hid the rollers; the model has the frame from the roller axis up with the lower half of the eight roller treads showing. The elevation is redrawn to that: frame from y 0.215, eight roller discs."""
import sys
exec(open('rework-10-lib.py').read())
c = Catalog('excavator')
shapes = c.asset['representations']['200']['geometry']['elevation']
frame = next(s for s in shapes if s['id'] == 'track-frame')
frame['d'] = "M -1.7,0.215 L 1.5,0.215 L 1.5,0.62 L -1.7,0.62 Z"
if not any(s['id'] == 'roller-1' for s in shapes):
    at = shapes.index(frame)
    for k in range(8):
        shapes.insert(at, {"id": f"roller-{k + 1}", "bone": "undercarriage", "role": "detail", "z": frame['z'] - 0.5 + k * 0.01, "filled": True, "tone": 0.3, "kind": "ellipse", "cx": round(-1.42 + 0.4057 * k, 4), "cy": 0.215, "rx": 0.09, "ry": 0.09, "region": "tracks", "material": "track-steel"})
c.touch('representations')
print(c.write(candidate='--candidate' in sys.argv))
