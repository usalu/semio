"""🧩 Author closed neutral traversal probes independently of full-view applicability."""
from pathlib import Path
import json
p=Path(__file__).parent
cases=[]
for id,status,members,expected in [('unique-match','unique',['match'],'pass'),('unique-empty','unique',[],'fail'),('absent-declaration','absent',[],'incomplete'),('ambiguous-owner','ambiguous',['match'],'incomplete'),('late-unknown-after-match','unknown',['match'],'incomplete'),('known-nonmember','unique',['nonmember'],'fail'),('unknown-member','unique',['unknown'],'incomplete'),('match-then-unknown-member','unique',['match','unknown'],'incomplete')]:
 cases.append({'id':id,'queryStatus':status,'memberStates':members,'probe':'exists-matching-entity','expected':expected})
data={'kind':'mvd-traversal-kernel-probes','dialect':'normalized-probe-only','schema':'IFC4X3_ADD2','releasedViewAuthority':False,'wholeViewEvaluated':False,'rule':{'declaringEntity':'IFCELEMENT','attribute':'FillsVoids','entity':'IFCRELFILLSELEMENT','next':{'declaringEntity':'IFCRELFILLSELEMENT','attribute':'RelatingOpeningElement','entity':'IFCOPENINGELEMENT'}},'cases':cases}
schema={'$schema':'https://json-schema.org/draft/2020-12/schema','type':'object','additionalProperties':False,'required':list(data),'properties':{'kind':{'const':data['kind']},'dialect':{'const':data['dialect']},'schema':{'const':data['schema']},'releasedViewAuthority':{'const':False},'wholeViewEvaluated':{'const':False},'rule':{'const':data['rule']},'cases':{'type':'array','minItems':8,'maxItems':8,'items':{'type':'object','additionalProperties':False,'required':['id','queryStatus','memberStates','probe','expected'],'properties':{'id':{'type':'string','minLength':1},'queryStatus':{'enum':['unique','absent','ambiguous','unknown']},'memberStates':{'type':'array','items':{'enum':['match','nonmember','unknown']}},'probe':{'const':'exists-matching-entity'},'expected':{'enum':['pass','fail','incomplete']}}}}}
}
(p/'mvd-traversal-kernel-probes.json').write_text(json.dumps(data,indent=2),encoding='utf8');(p/'mvd-traversal-kernel-probes.schema.json').write_text(json.dumps(schema,indent=2),encoding='utf8')

