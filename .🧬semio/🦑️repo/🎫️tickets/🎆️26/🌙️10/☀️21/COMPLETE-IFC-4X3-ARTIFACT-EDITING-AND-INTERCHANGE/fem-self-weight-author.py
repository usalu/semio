"""🏋️ Author the closed self-weight projection contract and public endpoint laws."""
import json
from pathlib import Path

ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
root=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/fem/self-weight'
root.mkdir(parents=True,exist_ok=True)
rows=[
 dict(id='unset',kind='unset',expected=False),dict(id='zero',kind='real',values=[0,0,0],expected=False),dict(id='signed-zero',kind='real',values=[-0.0,0,-0.0],expected=False),
 dict(id='down',kind='real',values=[0,0,-1],expected=True),dict(id='typed-down',kind='typed',values=[0,0,-1]),
 dict(id='up',kind='real',values=[0,0,1]),dict(id='horizontal',kind='real',values=[1,0,0]),dict(id='fraction',kind='real',values=[0,0,-0.5]),
 dict(id='short',kind='real',values=[0,-1]),dict(id='long',kind='real',values=[0,0,-1,0]),dict(id='integer',kind='integer',values=[0,0,-1]),
 dict(id='wrong-typed',kind='wrong-typed',values=[0,0,-1]),dict(id='derived',kind='derived'),dict(id='scalar',kind='scalar'),dict(id='missing',kind='missing'),dict(id='component-unset',kind='component-unset',values=[0,0,-1])]
(root/'cases.json').write_text(json.dumps(dict(version=1,cases=rows),indent=2)+'\n',encoding='utf-8')
schema={'$schema':'https://json-schema.org/draft/2020-12/schema','type':'object','additionalProperties':False,'required':['version','cases'],'properties':{'version':{'const':1},'cases':{'type':'array','minItems':16,'items':{'type':'object','additionalProperties':False,'required':['id','kind'],'properties':{'id':{'type':'string','minLength':1},'kind':{'enum':['unset','real','typed','wrong-typed','integer','derived','scalar','missing','component-unset']},'values':{'type':'array','items':{'type':'number'}},'expected':{'type':'boolean'}}}}}}
(root/'schema.json').write_text(json.dumps(schema,indent=2)+'\n',encoding='utf-8')
(root/'self-weight.feature').write_text('Feature: Exact per-case self-weight projection\n  Scenario: upright implicit weight\n    Given a case with true FEM selfWeight and explicit SI material and section data\n    When exported to IFC4x3 and projected back\n    Then SelfWeightCoefficients is [0,0,-1] without synthesized actions\n    And the case flag and original explicit loads are retained\n  Scenario: unsupported gravity vector\n    Given a valid IFC coefficient vector outside zero or upright unit gravity\n    Then analytical projection refuses without publishing a partial model\n',encoding='utf-8')
print('[DEBUG] self-weight contract '+str(len(rows)))
