"""🪵 Bind validated original-sized timber stock into canonical bilingual native source membership."""
from copy import deepcopy


def with_skirting(api,source,elements):
    """📦 Construct an isolated source value with explicit material and storey ownership."""
    model=deepcopy(source);tags=[obj['tag'] for obj in elements]
    if len(tags)!=len(set(tags)) or any(tag in {e['tag'] for e in model['elements']} for tag in tags):raise ValueError('Skirting publication requires distinct new products')
    material=deepcopy(api._find(model,'MAT-PAINTED-WOOD'));tag='MAT-SKIRTING-GRAY'
    if any(item['tag']==tag for item in model['materials']):raise ValueError('The new timber material already exists')
    material.update(tag=tag,guid=api.global_id(tag),name={'en':'Original gray painted timber skirting; species and pigment unmeasured','de':'Ursprüngliche graue gestrichene Holzsockelleiste; Holzart und Pigment unvermessen'})
    material['surface_finish']={'binder':'unmeasured_paint','colour_family':'gray','substrate':'MAT-PAINTED-WOOD','quantity_scope':'existing_substrate_envelope','film_thickness':'unmeasured','basis':'inferred','source':'SAB2024-p190 original15×90mm rounded timber skirting and gray finish; original1930 hall/salon photographs; binder/pigment unmeasured','display_rgb':{'value':[.47,.48,.49],'basis':'inferred','source':'Illustrative gray; historic RGB unmeasured'}}
    model['materials'].append(material);architecture=next(item for item in model['models'] if item['id']=='architecture')
    for obj in deepcopy(elements):
        if obj['guid']!=api.global_id(obj['tag']) or obj['class']!='IfcCovering' or obj['predefined_type']!='SKIRTINGBOARD' or obj['material']!=tag:raise ValueError('Skirting identity/class/material must match its authored construction')
        model['elements'].append(obj);architecture['element_tags'].append(obj['tag'])
        relation=next(item for item in model['relations'] if item['class']=='IfcRelContainedInSpatialStructure' and item['relating']==obj['storey'])
        relation['related'].append(obj['tag'])
    relation_tag='REL-MAT-'+tag
    model['relations'].append({'guid':api.global_id(relation_tag),'tag':relation_tag,'class':'IfcRelAssociatesMaterial','relating':tag,'related':tags,'name':{'en':'Gray painted timber material is assigned','de':'Grau gestrichenes Holzmaterial ist zugewiesen'}})
    return model
