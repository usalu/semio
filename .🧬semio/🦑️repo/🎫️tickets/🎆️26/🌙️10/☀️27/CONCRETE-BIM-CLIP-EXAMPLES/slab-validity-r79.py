import hashlib,time
api=module.api
model79=api.load_model()
scan79={'source':hashlib.sha256(Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json').read_bytes()).hexdigest(),'slabs':[],'invalid':[]}
started79=time.monotonic()
for slab79 in [e for e in model79['elements'] if e['class']=='IfcSlab']:
    g79=api._slab_geometry(model79,slab79)
    counts79={}
    for key79 in ('parts','full_parts','shared_parts','finish_parts','full_finish_parts','shared_finish_parts'):
        parts79=g79[key79];counts79[key79]=len(parts79)
        for index79,part79 in enumerate(parts79):
            poly79=module.Polygon(part79['outline'],part79.get('holes',[]))
            if not poly79.is_valid or poly79.area<=0:
                scan79['invalid'].append({'slab':slab79['tag'],'view':key79,'index':index79,'outline':part79['outline'],'area':poly79.area})
    scan79['slabs'].append({'tag':slab79['tag'],'counts':counts79})
    Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES/🗑️generated/slab-validity-progress-r79.json').write_text(json.dumps({'debug':'[DEBUG] native slab topology scan','completed':len(scan79['slabs']),'tag':slab79['tag'],'invalid':len(scan79['invalid'])}),encoding='utf8')
scan79['seconds']=time.monotonic()-started79
Path('/repo/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️27/CONCRETE-BIM-CLIP-EXAMPLES/🗑️generated/slab-validity-r79.json').write_text(json.dumps(scan79,ensure_ascii=False),encoding='utf8')
print({'slabs':len(scan79['slabs']),'invalid':len(scan79['invalid']),'seconds':scan79['seconds']})
