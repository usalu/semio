"""🪭 Install the neutral radial fold contract in both schema representations."""
from pathlib import Path
import json
S=next(Path('.').glob('🎓*/*/*staffage/🧬*'))
p=S/'🔣️.json';before=p.read_text();data=json.loads(before)
bone=data['$defs']['Bone'];bone['properties']['abductionDeg']={'type':'number','minimum':-180,'maximum':180,'description':'Rest rotation about the parent normal, applied after the in-plane angle.'}
f=data['$defs']['Rig']['properties'];f['radialFolds']={'type':'array','items':{'type':'object','additionalProperties':False,'required':['left','right','facets','cutAngleDeg'],'properties':{'left':{'$ref':'#/$defs/Slug'},'right':{'$ref':'#/$defs/Slug'},'facets':{'type':'array','minItems':2,'uniqueItems':True,'items':{'$ref':'#/$defs/Slug'}},'cutAngleDeg':{'type':'number','exclusiveMinimum':0,'exclusiveMaximum':180}}},'description':'Ordered pairs of terminal facet frames share a crown. The total angle of the flat fabric fixes the width of each pleat.'}
def replace_object(text,name,new):
    old=json.loads(text)['$defs'][name];old=json.dumps(old,ensure_ascii=False,indent=2).replace('\n','\n    ');new=json.dumps(new,ensure_ascii=False,indent=2).replace('\n','\n    ');assert old in text;return text.replace(old,new,1)
new=replace_object(before,'Bone',bone);new=replace_object(new,'Rig',data['$defs']['Rig']);assert p.read_text()==before
p.write_text(new)
p=S/'🟦️.ts';s=p.read_text();k=s.index('export type Bone = {');k=s.index('  readonly angleDeg: number;',k)+len('  readonly angleDeg: number;');s=s[:k]+'\n  readonly abductionDeg?: number;'+s[k:];k=s.index('/** 🌳️ A transform hierarchy');s=s[:k]+'''/** 🪭 Ordered rigid facets between two moving rays. Each successive pair shares a crease and preserves its flat cut angle. */
export type RadialFold = {
  readonly left: Slug;
  readonly right: Slug;
  readonly facets: readonly Slug[];
  readonly cutAngleDeg: number;
};

'''+s[k:];k=s.index('  readonly bones: readonly Bone[];',s.index('export type Rig = {'))+len('  readonly bones: readonly Bone[];');s=s[:k]+'\n  readonly radialFolds?: readonly RadialFold[];'+s[k:];p.write_text(s)
