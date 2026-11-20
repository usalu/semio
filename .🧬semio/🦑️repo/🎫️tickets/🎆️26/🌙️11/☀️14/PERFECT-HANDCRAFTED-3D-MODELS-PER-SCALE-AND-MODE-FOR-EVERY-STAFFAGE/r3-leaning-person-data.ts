/** 📐 Shared data of the round 3 model of `humans/leaning-person`: rig segment lengths (Winter 2009, stature 1.72 m, 20 mm sneaker soles) and the phone held in the right hand.
 * @see ./r3-leaning-person-roots.ts */
export const RIG = { sole: 0.021, ankle: 0.088, torso: 0.4755, head: 0.249, upperArm: 0.318, forearm: 0.252, hand: 0.186, thigh: 0.4202, shank: 0.4105, shoulderU: 0.4202, shoulderZ: 0.175, hipZ: 0.09 } as const;
/** 📱 The phone in the right hand (palm turned to +w, thumb ahead), hand-local: centre, long axis +v (towards the thumb), screen normal +w; the fingers wrap its far long edge. */
export const PHONE = { u: 0.097, v: 0.03, w: 0.0205, length: 0.147, width: 0.0715, thick: 0.0078 } as const;
