const g = 9.81;
const th = Math.atan2(2.9, 3.6) * 180 / Math.PI;
const L = Math.hypot(3.6, 2.9);
console.log("stair angleDeg", th.toFixed(6), "L", L.toFixed(6), "qSlope", (3.0*0.6*3.6/L).toFixed(3), "qHoriz", (3.0*0.6).toFixed(1));
const c = Math.cos(th*Math.PI/180), s = Math.sin(th*Math.PI/180);
const r = 2.9/12;
const pts = [[0,0]];
for (let i=0;i<12;i++){ pts.push([0.3*i, r*(i+1)]); pts.push([0.3*(i+1), r*(i+1)]); }
const local = pts.map(([x,y]) => [x*c + y*s, -x*s + y*c].map(v => +v.toFixed(4)));
console.log("stairLocal", JSON.stringify(local));
console.log("balcony slab kg", 0.2*1.0*1.5*2500, "post kg", 7.36e-4*7850*1.2, "post N", 7.36e-4*7850*1.2*g, "slab N", 750*g);
console.log("glulam kg", 0.096*420*8, "N", 0.096*420*8*g);
console.log("column kg", 7.81e-3*7850*3, "N", 7.81e-3*7850*3*g);
console.log("stringer kg", 3.88e-3*7850*L + 25.4, "N", (3.88e-3*7850*L + 25.4)*g);
console.log("door track kg", 1.136e-3*7850*3, "N", 1.136e-3*7850*3*g, "leaf kg", 50+10, "N", 60*g);
console.log("tilt sash kg", 2500*0.024*0.9*1.2 + 8, "N", (2500*0.024*0.9*1.2 + 8)*g);
console.log("balustrade kg", 0.0215*2500*1.1, "N", 0.0215*2500*1.1*g);
console.log("canopy kg", 2.01e-3*7850*5.5, "N", 2.01e-3*7850*5.5*g);
console.log("HEA160 kg/m", 3.88e-3*7850);
