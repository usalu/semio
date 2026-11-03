import re
p='🎭️cast-export/🔣️.json'
s=open(p,encoding='utf-8',newline='').read()
lines=[l for l in s.split('\n') if not re.match(r'\s*"(commuter-plan-mirrored|window-front-mirrored)": ',l)]
s='\n'.join(lines)
s=s.replace(', "commuter-plan-mirrored": "plan"','').replace(', "window-front-mirrored": "front"','')
s=s.replace(', "commuter-plan-mirrored": "walker-line"','')
s=s.replace('"mirrorOf": { "walker-line-mirrored": "walker-line", "commuter-plan-mirrored": "commuter-plan", "window-front-mirrored": "window-front" }','"mirrorOf": { "walker-line-mirrored": "walker-line" }')
def add(m):
    t=m.group(0)
    return t if '"viewpoint"' in t else t.replace('"theme"','"viewpoint": "elevation", "theme"',1) if '"theme"' in t else t
s=re.sub(r'"cast": \{[^{}]*\}',add,s)
s=s.replace('"invalid": [\n','"invalid": [\n    { "name": "a mirrored plan", "cast": { "asset": "walking-commuter", "clip": "walk-cycle", "view": "dark-line", "viewpoint": "plan", "theme": "dark", "mirror": true }, "issue": "mirror needs viewpoint elevation" },\n    { "name": "a mirrored front", "cast": { "asset": "tilt-window", "pose": "tilted-open", "view": "dark-line", "viewpoint": "front", "theme": "dark", "mirror": true }, "issue": "mirror needs viewpoint elevation" },\n    { "name": "missing viewpoint", "cast": { "asset": "walking-commuter", "pose": "standing", "view": "dark-line", "theme": "dark" }, "issue": "viewpoint" },\n',1)
open(p,'w',encoding='utf-8',newline='').write(s)
