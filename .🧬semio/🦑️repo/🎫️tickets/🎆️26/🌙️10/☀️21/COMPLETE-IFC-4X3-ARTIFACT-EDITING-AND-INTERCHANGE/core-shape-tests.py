from pathlib import Path
import json
R=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️tests/🧬️editing')
value={'kind':'integer','value':'3'}
complex_=[{'name':'IFCROOT','args':[{'kind':'string','value':'Root'}]}]
entity={'id':'3','name':'IFCBLOB','args':[{'kind':'unset'},{'kind':'derived'},value,{'kind':'real','value':{'bits':'0'}},{'kind':'string','value':'Text'},{'kind':'binary','value':'17'},{'kind':'enum','value':'T'},{'kind':'reference','value':'1'},{'kind':'aggregate','value':[value]},{'kind':'typedValue','value':{'name':'IFCLABEL','items':[{'kind':'string','value':'Label'}]}}],'complex':complex_}
base={'schema':'stdio.ifc','header':{'fileDescription':[{'kind':'string','value':'Input'}],'fileName':[{'kind':'string','value':'input.ifc'}],'fileSchema':[{'kind':'aggregate','value':[{'kind':'string','value':'IFC4X3_ADD2'}]}]},'entities':[{'id':'1','name':'IFCPERSON','args':[{'kind':'integer','value':'1'},{'kind':'aggregate','value':[{'kind':'integer','value':'2'}]}],'complex':complex_},{'id':'2','name':'IFCPERSON','args':[],'complex':[]}]}
mutations=[{'mutation':'setFileDescription','values':[{'kind':'string','value':'Changed'}]},{'mutation':'setFileName','values':[{'kind':'string','value':'changed.ifc'}]},{'mutation':'setFileSchema','values':base['header']['fileSchema']},{'mutation':'insertEntity','index':1,'entity':entity},{'mutation':'removeEntity','id':'2'},{'mutation':'setEntityName','id':'1','name':'IFCWALL'},{'mutation':'setEntityArg','id':'1','index':0,'value':value},{'mutation':'insertEntityArg','id':'1','index':1,'value':value},{'mutation':'removeEntityArg','id':'1','index':1},{'mutation':'setEntityComplex','id':'1','complex':[]},{'mutation':'setEntityValue','id':'1','index':1,'path':[0],'value':value}]
diff={'fileDescription':[{'kind':'string','value':'Changed'}],'fileName':[{'kind':'string','value':'changed.ifc'}],'fileSchema':base['header']['fileSchema'],'entities':{'removed':[{'id':'2','index':1}],'modified':[{'id':'1','diff':{'name':'IFCWALL','args':{'removed':[1],'modified':[{'index':0,'value':value}],'added':[{'index':1,'value':{'kind':'binary','value':'17'}}]},'complex':complex_}}],'added':[{'index':1,'entity':entity}]}}
(R/'shape-cases.json').write_text(json.dumps({'base':base,'mutations':mutations,'diff':diff},ensure_ascii=False,indent=2)+'\n',encoding='utf-8',newline='\n')
p=R/'🥒️.feature';s=p.read_text(encoding='utf-8');s+='''

  Scenario Outline: Native shape admission agrees with the declared schema
    Given each of the eleven mutation branches and every header entity and list diff lane
    When <variation> changes each required or optional field
    Then synchronous and asynchronous admission agree with the native schema oracle
    And refused commands publish no event and preserve their original sources
    Examples:
      | variation |
      | missing |
      | undefined |
      | null |
      | wrong type |
      | extra property |
      | array hole |
      | undefined array item |
  Scenario: Every published recorded command satisfies its serialized transport schema
    Given a structurally admitted command
    When history publishes its event
    Then its recorded native and decimal JSON command shapes are valid
''';p.write_text(s,encoding='utf-8',newline='\n')
