"""🪵 Bind an explicitly unpublished wall cohort and completely rebuilt native stock."""
import hashlib,json,runpy
from copy import deepcopy
from pathlib import Path

def prepare(api,source,report):
    """🔐 Require exact author helper, current source and unchanged canonical candidate model."""
    helper=Path(report['unpublished_model_helper']);assert helper.is_file() and report['source_stable'] and not report['stable']
    model=runpy.run_path(str(helper))['candidate'](api,source)
    assert hashlib.sha256(json.dumps(model,sort_keys=True,separators=(',',':'),allow_nan=False).encode()).hexdigest()==report['model_canonical_sha256']
    elements=deepcopy(report['elements']);tags=[obj['tag'] for obj in elements];assert len(tags)==len(set(tags))
    assert not set(tags)&{obj['tag'] for obj in model['elements']}
    model['elements'].extend(elements);next(item for item in model['models'] if item['id']=='architecture')['element_tags'].extend(tags)
    for obj in elements:
        assert obj['guid']==api.global_id(obj['tag']) and obj['class']=='IfcCovering' and obj['predefined_type']=='SKIRTINGBOARD' and obj['material']=='MAT-SKIRTING-GRAY'
        next(item for item in model['relations'] if item['class']=='IfcRelContainedInSpatialStructure' and item['relating']==obj['storey'])['related'].append(obj['tag'])
    tag='REL-MAT-MAT-SKIRTING-GRAY';assert not any(item['tag']==tag for item in model['relations'])
    model['relations'].append({'guid':api.global_id(tag),'tag':tag,'class':'IfcRelAssociatesMaterial','relating':'MAT-SKIRTING-GRAY','related':tags,'name':{'en':'Gray painted timber material is assigned','de':'Grau gestrichenes Holzmaterial ist zugewiesen'}})
    return model
