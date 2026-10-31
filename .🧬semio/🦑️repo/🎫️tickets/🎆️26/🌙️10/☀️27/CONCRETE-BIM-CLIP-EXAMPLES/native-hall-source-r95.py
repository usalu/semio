"""🏗️ Prepare an unpublished source-qualified service partition while retaining fixed fixture controls."""
from copy import deepcopy
from fractions import Fraction
from pathlib import Path
import runpy
INPUTS=[Path(__file__).with_name("roof-layer-binding-source-r95.py")]

def candidate(api,source):
    """🚶 Preserve the cabinet face and all original fixture axes while selecting the inferred thin wall."""
    model=deepcopy(source);runpy.run_path(str(INPUTS[0]))['apply'](model);wall=api._find(model,'W-G-SERVICE')
    assert wall['geometry']['path']==[[7.05,11.9],[7.05,18.2]] and wall['material']=='LS-PARTITION-15'
    assert api._find(model,'FIX-HALL-BASIN')['geometry']['axis']==[8.128,13.15]
    assert Fraction('7.015')-Fraction('.08')/2==Fraction('7.05')-Fraction('.15')/2
    wall['material']='LS-PARTITION-08';wall['type']='TY-PARTITION-08';wall['geometry']['path']=[[7.015,y] for _,y in wall['geometry']['path']]
    wall['attributes']['thickness']={'value':.08,'unit':'m','basis':'inferred','source':'SAB2024-p45-registered-service-partition;p96-thin-partition-family'}
    wall['metadata']['revision']+=1
    wall['historical_evidence']['sources'].append('SAB2024-p45-ground-service-partition-registration-r95')
    wall['historical_evidence']['uncertainty']['en']+=' Service partition8cm total and6cm masonry/two1cm plaster split are inferred from the thin author45 plan line, not a measured ground-wall section. Cabinet-side finishedfaceX6.975 and fixed hall basin/post axes retained; hall faceX7.055 supersedes the earlier wholly assumed15cm wall. Proud15×90mm boards and all actual door panels remain.'
    wall['historical_evidence']['uncertainty']['de']+=' Dienstbotenflurwand8cm mit6cmMauerwerk undzwei1cmPutzschichten aus dünnerPlanlinie45 abgeleitet, kein vermessener Erdgeschossquerschnitt. SchrankseitigeFertigflächeX6.975 undBecken-/Stützenachsen unverändert; HallenflächeX7.055 ersetzt frühere pauschale15cmAnnahme. Vorgelegte15×90mmLeisten undalleTürblätter bleiben.'
    for cls,target in (('IfcRelAssociatesMaterial','LS-PARTITION-08'),('IfcRelDefinesByType','TY-PARTITION-08')):
        for relation in model['relations']:
            if relation['class']==cls and 'W-G-SERVICE' in relation.get('related',[]):relation['related'].remove('W-G-SERVICE')
        relation=next(item for item in model['relations'] if item['class']==cls and item['relating']==target);relation['related'].append('W-G-SERVICE')
    for tag in ('W-G-ROOM-10','W-G-ROOM-13'):
        host=api._find(model,tag)
        for point in host['geometry']['path']:
            if isinstance(point,list) and point[0]==7.05:point[0]=7.015
        host['metadata']['revision']+=1
    for tag in ('SP-HALL','SP-SERVANTS-1','SP-SERVANTS-2'):
        room=api._find(model,tag)
        for point in room['outline']:
            if isinstance(point,list) and point[0]==7.05 and 11.9<=point[1]<=18.2:point[0]=7.015
    tags={obj['tag'] for obj in model['elements'] if obj.get('predefined_type')=='SKIRTINGBOARD'}
    assert tags
    model['elements']=[obj for obj in model['elements'] if obj['tag'] not in tags]
    for item in model['models']:item['element_tags']=[tag for tag in item['element_tags'] if tag not in tags]
    for relation in model['relations']:
        if 'related' in relation:relation['related']=[tag for tag in relation['related'] if tag not in tags]
    model['relations']=[relation for relation in model['relations'] if relation.get('related',True)]
    return model
