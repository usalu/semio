"""🧱 Declare shared editable roof finish dependencies without changing any baseline material cell."""
from copy import deepcopy


def apply(model):
    """📝 Add only explicit shared-layer recipe metadata to the three original roof finish products."""
    layer=next(v for m in model["materials"] if m["tag"]=="LS-ROOF" for v in m["layers"] if v["id"]=="paving")
    for tag,material,role in (("FIN-ROOF-PAVING","MAT-ROOF-FINISH","weathering_reserve"),("FIN-ROOF-GRAVEL","MAT-ROOF-FINISH","weathering_reserve"),("FIN-ROOF-STAIR","MAT-MORTAR-PUTTY","bedding")):
        obj=next(v for v in model["elements"] if v["tag"]==tag)
        obj["geometry"]["layer_binding"]={"material":"LS-ROOF","layer":"paving","reference_thickness":deepcopy(layer["thickness"]),"support_material":material,"support_role":role}
        if tag=="FIN-ROOF-PAVING":
            quantity=lambda value:{"value":value,"unit":"m","basis":"inferred","source":"Original explicit high30mm roof-cap material allocation; retain the unresolved15mm wearing choice through design layer edits"}
            obj["geometry"]["layer_binding"]["wearing_bands"]=[{"maximum_reference_capacity":quantity(.03),"depth":quantity(.015),"material":"MAT-PAVING-UNRESOLVED"}]
