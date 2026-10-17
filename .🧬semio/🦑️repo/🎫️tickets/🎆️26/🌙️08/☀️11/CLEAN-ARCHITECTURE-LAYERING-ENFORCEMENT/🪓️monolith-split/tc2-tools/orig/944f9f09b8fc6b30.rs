//! ↗️ Full affine composition and QR decomposition, including shear and reflections.
use crate::DrawingTransform;

pub fn drawing_transform_to_matrix(transform: &DrawingTransform) -> [f64;6] {
    let (s,c)=transform.rotation.sin_cos();
    [transform.scale_x*c,transform.scale_x*s,transform.shear*c-transform.scale_y*s,transform.shear*s+transform.scale_y*c,transform.x,transform.y]
}

pub fn drawing_matrix_to_transform(matrix: [f64;6]) -> DrawingTransform {
    let [a,b,c,d,x,y]=matrix;
    let scale_x=a.hypot(b);
    if scale_x==0.0 { return DrawingTransform { x,y,scale_x:0.0,scale_y:c.hypot(d),rotation:if c==0.0&&d==0.0 {0.0} else {(-c).atan2(d)},shear:0.0 }; }
    let (ux,uy)=(a/scale_x,b/scale_x);
    DrawingTransform { x,y,scale_x,scale_y:ux*d-uy*c,rotation:b.atan2(a),shear:ux*c+uy*d }
}

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
