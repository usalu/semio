fn geometry_leaf(path: &str) -> bool {
    let Some((id, leaf)) = path.strip_prefix("geometry.objects.").and_then(|rest| rest.split_once('.')) else { return false };
    let indexed = leaf.ends_with(']');
    let node = leaf.starts_with("shape.") || leaf.starts_with("symbolic.");
    !id.is_empty()
        && ((node && (leaf.contains(".parameters.") || (indexed && (leaf.contains(".translation[") || leaf.contains(".rotationDeg["))))) || (indexed && ((leaf.starts_with("spaces[") || leaf.starts_with("surfaces[")) && leaf.contains(".bounds.") || leaf.starts_with("ports[") && (leaf.contains(".position[") || leaf.contains(".direction[")))))
}


fn main(){
 for p in ["geometry.objects.g1.shape.parameters.width","geometry.objects.g1.shape.child.children[1].translation[2]","geometry.objects.g1.shape.rotationDeg[0]","geometry.objects.g1.spaces[0].bounds.max[1]","geometry.objects.g1.ports[0].position[2]","geometry.objects.g1.shape","geometry.objects.g1.shape.node","geometry.objects.g1.id","geometry.objects.g1.spaces[0].kind","geometry.objects.g1.shape.kind"]{println!("{p} {}",geometry_leaf(p));}
}