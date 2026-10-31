def _interface_graph(api,model,obj):
    """🧭 Reject cyclic independently occupied material reservations before querying their owners."""
    visiting=set();visited=set()
    def visit(current):
        tag=current["tag"]
        if tag in visiting:raise ValueError("Independent structural interface ownership cycle")
        if tag in visited:return
        visiting.add(tag)
        refs=[*current["geometry"].get("interface_owners",()),*(joint["owner"] for joint in current["geometry"].get("structural_joints",()))]
        if len(refs)!=len(set(refs)):raise ValueError("Independent structural interface owners must be distinct")
        for key in refs:visit(api._find(model,key))
        visiting.remove(tag);visited.add(tag)
    visit(obj)


def reserve_interfaces(api,model,obj,parts,progress=None,cancel=None):
    """🤝 Reserve explicitly declared independent occupied material in a receiving member of any material."""
    _interface_graph(api,model,obj)
    stocks=[{**deepcopy(p),"material":_material(api,model,obj,p),"role":p.get("role","structural_member")} for p in parts]
    owners=[]
    for key in obj["geometry"]["interface_owners"]:
        owner=api._find(model,key)
        if owner["class"]=="IfcSlab":cells=api._slab_geometry_entry(model,owner,exact=True)["parts"]
        elif owner["geometry"].get("structural_joints"):cells=api._solid_geometry(model,owner,resolve_embedded=False,cancel=cancel,exact=True)["parts"]
        else:cells=api._interface_owner_parts(model,key)
        owners.append((owner,cells))
    for index,(owner,cells) in enumerate(owners):
        for other,others in owners[:index]:
            for stock in stocks:
                for cell in cells:
                    for common in _intersection(api,stock,cell,cancel):
                        if any(_intersection(api,common,p,cancel) for p in others):raise ValueError("Independent structural interface owners must have exclusive physical material")
    shared=[]
    for index,(owner,cells) in enumerate(owners):
        if cancel and cancel():raise InterruptedError("Receiving material reservation cancelled")
        if progress:progress({"phase":"receiving-material","completed":index,"total":len(owners)})
        for stock in stocks:
            for cell in cells:
                for common in _intersection(api,stock,cell,cancel):
                    shared.append({**common,"owner":owner["tag"],"owner_guid":owner["guid"],"joint_host":obj["tag"],"material":_material(api,model,owner,cell),"role":"independent_material_reference"})
    cutters=[p for owner,cells in owners for p in cells]
    current=[cell for stock in stocks for cell in api._reference_material_parts(stock,cutters,cancel=cancel)]
    def normalized(items):
        """📐 Preserve exact authoritative meshes while exposing their finite axial extents to slab consumers."""
        result=[]
        for part in items:
            vertices=api.part_vertices(part)
            result.append({**part,"outline":api._convex_hull([[p[0],p[1]]for p in vertices]),"z0":min(p[2]for p in vertices),"z1":max(p[2]for p in vertices)})
        return result
    return normalized(current),normalized(shared)
