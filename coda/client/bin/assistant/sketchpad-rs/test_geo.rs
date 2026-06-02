use geo::{polygon, Polygon, MultiPolygon};
use geo::algorithm::area::Area;
use geo::algorithm::euclidean_length::EuclideanLength;
use geo_booleanop::boolean::BooleanOp;

fn main() {
    let p1 = polygon![
        (x: 0.0, y: 0.0),
        (x: 10.0, y: 0.0),
        (x: 10.0, y: 10.0),
        (x: 0.0, y: 10.0),
        (x: 0.0, y: 0.0),
    ];
    let p2 = polygon![
        (x: 5.0, y: 0.0),
        (x: 15.0, y: 0.0),
        (x: 15.0, y: 10.0),
        (x: 5.0, y: 10.0),
        (x: 5.0, y: 0.0),
    ];
    
    let mut unioned: MultiPolygon<f64> = MultiPolygon::new(vec![p1.clone()]);
    unioned = unioned.union(&p2);
    
    let mut perimeter = 0.0;
    for p in unioned.iter() {
        perimeter += p.exterior().euclidean_length();
    }
    
    println!("Area: {}, Perimeter: {}", unioned.unsigned_area(), perimeter);
}
