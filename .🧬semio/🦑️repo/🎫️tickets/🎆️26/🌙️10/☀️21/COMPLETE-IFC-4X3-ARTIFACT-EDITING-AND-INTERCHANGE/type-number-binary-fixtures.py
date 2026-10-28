from pathlib import Path
import json,struct
p=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔎️details/🧪️tests')
u=lambda:{'kind':'unset'}
s=lambda text:{'kind':'string','value':text}
r=lambda text:{'kind':'reference','value':text}
b=lambda text:{'kind':'binary','value':text}
e=lambda text:{'kind':'enum','value':text}
f=lambda bits:{'kind':'real','value':{'bits':bits}}
i=lambda text:{'kind':'integer','value':text}
a=lambda *items:{'kind':'aggregate','value':list(items)}
part=lambda name,*args:{'name':name,'args':list(args)}
entity=lambda id,name,args,complex=[]:{'id':id,'name':name,'args':args,'complex':complex}
points=[entity('9','IFCCARTESIANPOINT',[a(f('0'),f('0'))]),entity('10','IFCCARTESIANPOINT',[a(f('4607182418800017408'),f('0'))])]
sources={'real':{'entities':[entity('1','IFCPOINTONCURVE',[r('7'),u()],[part('IFCPOINT'),part('IFCGEOMETRICREPRESENTATIONITEM'),part('IFCREPRESENTATIONITEM')]),entity('7','IFCPOLYLINE',[a(r('9'),r('10'))]),*points]},'number':{'entities':[entity('1','IFCQUANTITYNUMBER',[u(),s('Caller formula')],[part('IFCPHYSICALQUANTITY',s('Caller number'),u()),part('IFCPHYSICALSIMPLEQUANTITY',u())])]},'binary':{'entities':[entity('1','IFCBLOBTEXTURE',[s('PNG'),u()],[part('IFCSURFACETEXTURE',e('T'),e('F'),s('TEXTURE'),u(),u()),part('IFCPRESENTATIONITEM')])]},'ratio':{'entities':[entity('1','IFCCOLOURRGB',[u(),f('0'),f('4607182418800017408')],[part('IFCCOLOURSPECIFICATION',s('Caller colour')),part('IFCPRESENTATIONITEM')])]}}
cases=[]
for id,bits in [('zero','0'),('negative-zero','9223372036854775808'),('half','4602678819172646912'),('one-and-half','4609434218613702656'),('subnormal','1'),('max-finite','9218868437227405311')]:cases.append({'id':'real-'+id,'source':'real','target':'IFCPOINTONCURVE','index':1,'value':f(bits),'args':[r('7'),f(bits)],'admitted':True})
for id,value in [('integer',i('2')),('binary64',f('4609434218613702656')),('large-integer',i('9007199254740993'))]:cases.append({'id':'number-'+id,'source':'number','target':'IFCQUANTITYNUMBER','index':0,'value':value,'args':[value,s('Caller formula')],'admitted':True})
for id,word in [('bytes','0A5FF'),('empty','0'),('leading-zeros','00000')]:cases.append({'id':'binary-'+id,'source':'binary','target':'IFCBLOBTEXTURE','index':1,'value':b(word),'args':[s('PNG'),b(word)],'admitted':True})
for id,bits,admitted in [('half','4602678819172646912',True),('above-one','4609434218613702656',False)]:cases.append({'id':'ratio-'+id,'source':'ratio','target':'IFCCOLOURRGB','index':0,'value':f(bits),'args':[f(bits),f('0'),f('4607182418800017408')],'admitted':admitted})
contract={'schema':'IFC4X3_ADD2','nativeExecution':False,'encoding':'ifc-template-command','sources':sources,'cases':cases,'invalidBits':['','-','01','-0','18446744073709551616','9218868437227405312','9221120237041090560','18442240474082181120'],'invalidBinary':['','4F','0a5','3A'],'binarySyntax':[{'word':word,'bits':bits}for word,bits in [('0',''),('30','0'),('31','1'),('23','11'),('12A','0101010'),('0A5FF','1010010111111111'),('00','0000'),('23B','111011'),('092A','100100101010')]],'binaryCandidateRefusals':['31','23','12A'],'numberModes':['integer','real']}
(p/'type-number-binary-input.json').write_text(json.dumps(contract,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
base=json.loads((p/'type-enum-input.schema.json').read_text(encoding='utf8'));defs=base['definitions'];defs['value']['oneOf'].append({'type':'object','additionalProperties':False,'required':['kind','value'],'properties':{'kind':{'const':'binary'},'value':{'type':'string'}}})
defs['value']['oneOf'][4]['properties']['value']['pattern']='^(0|-?[1-9][0-9]{0,18})$'
closed=lambda properties:{'type':'object','additionalProperties':False,'required':list(properties),'properties':properties}
text={'type':'string'};strings={'type':'array','items':text}
schema={'$schema':'http://json-schema.org/draft-07/schema#',**closed({'schema':{'const':'IFC4X3_ADD2'},'nativeExecution':{'const':False},'encoding':{'const':'ifc-template-command'},'sources':{'type':'object','additionalProperties':closed({'entities':{'type':'array','minItems':1,'items':{'$ref':'#/definitions/entity'}}})},'cases':{'type':'array','minItems':14,'items':closed({'id':text,'source':{'enum':['real','number','binary','ratio']},'target':text,'index':{'type':'integer','minimum':0},'value':{'$ref':'#/definitions/value'},'args':{'type':'array','items':{'$ref':'#/definitions/value'}},'admitted':{'type':'boolean'}})},'invalidBits':strings,'invalidBinary':strings,'binarySyntax':{'type':'array','items':closed({'word':text,'bits':{'type':'string','pattern':'^[01]*$'}})},'binaryCandidateRefusals':strings,'numberModes':{'const':['integer','real']}}),'definitions':defs}
(p/'type-number-binary-input.schema.json').write_text(json.dumps(schema,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print('[DEBUG]',len(cases),'neutral numeric/binary candidate cases authored before production')
