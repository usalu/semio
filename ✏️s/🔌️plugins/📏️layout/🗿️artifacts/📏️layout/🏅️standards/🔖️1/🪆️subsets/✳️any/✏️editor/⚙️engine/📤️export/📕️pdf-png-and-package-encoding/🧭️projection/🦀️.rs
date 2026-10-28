//! 🧭️ Local unrounded coordinates compose with PDF page projection without subtracting each local ordinate from a large page height.
pub fn path_matrix(a:[f64;6],height:f64)->[f64;6]{[a[0],-a[1],a[2],-a[3],a[4],height-a[5]]}
pub fn text_matrix(a:[f64;6],origin:[f64;2],height:f64,italic:bool)->[f64;6]{let skew=if italic{0.22}else{0.0};[a[0],-a[1],a[0]*skew-a[2],a[3]-a[1]*skew,a[0]*origin[0]+a[2]*origin[1]+a[4],height-(a[1]*origin[0]+a[3]*origin[1]+a[5])]}
pub fn image_matrix(a:[f64;6],rect:[f64;4],height:f64)->[f64;6]{let[x,y,width,h]=rect;[a[0]*width,-a[1]*width,-a[2]*h,a[3]*h,a[0]*x+a[2]*(y+h)+a[4],height-(a[1]*x+a[3]*(y+h)+a[5])]}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
