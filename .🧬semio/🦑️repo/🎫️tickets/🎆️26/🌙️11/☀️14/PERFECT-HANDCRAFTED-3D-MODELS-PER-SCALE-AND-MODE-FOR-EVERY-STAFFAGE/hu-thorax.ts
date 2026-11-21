/** 🫁 Costovertebral heights, lateral reaches and anterior courses in the two figures' torso frames, in millimetres. */
type Row = readonly [height: number, back: number, width: number, frontHeight: number, front: number]
type Point = [number, number, number]
type Figure = 'tour-guide' | 'person-with-umbrella'
type Scale = 50 | 100 | 200 | 500
const profiles: Record<Figure, Row[]> = {
  'tour-guide': [
    [433,50,66,418,-61], [419,54,88,395,-76], [402,58,107,371,-93], [384,62,122,346,-101],
    [365,64,135,321,-108], [346,67,144,295,-115], [326,68,148,268,-117],
    [305,67,148,241,-116], [282,66,144,217,-111], [259,62,136,194,-103],
    [234,57,119,183,-81], [210,50,91,177,-54],
  ],
  'person-with-umbrella': [
    [392,53,59,373,-71], [378,56,78,350,-83], [362,58,99,326,-96], [345,61,115,301,-104],
    [327,64,129,275,-110], [308,63,136,248,-117], [288,63,138,220,-117],
    [267,62,136,191,-116], [245,58,131,167,-110], [223,53,123,147,-102],
    [200,47,107,137,-79], [177,40,83,133,-50],
  ],
}
const levels = {50:[1,2,3,4,5,6,7,8,9,10,11,12],100:[1,3,5,7,9,11],200:[2,7,10],500:[2,7]}
const metre = (mm: number) => Math.round(mm * 100) / 100000
const tube = (id: string, path: Point[], radius: number, slender = false) => ({
  id, kind:'loft', bone:'torso', material:'bone', role:'body', tone:0.9,
  sections:path.map(at=>({at:at.map(metre),rx:metre(radius),ry:metre(radius)})),
  capStart:true,capEnd:true,...(slender?{slender:true}:{}),member:'torso',part:'member',
})

/** 🦴 True ribs meet the sternum. The costal margin shares cartilage and the last two pairs end on the flank. */
export function ribCage(figure: Figure, scale: Scale) {
  const rows=profiles[figure],seventh=rows[6],solids=[]
  for(const n of levels[scale]) {
    const [h,back,width,frontHeight,front]=rows[n-1],drop=h-frontHeight,depth=back-front
    const point=(fall:number,reach:number,side:number):Point=>[h-fall*drop,back-reach*depth,side*width]
    const head:Point=[h,back,4],angle=point(.12,-.08,.46),flank=point(.52,.39,1)
    const anterior=point(.84,.82,.80),cartilage:Point=[frontHeight,front+.055*depth,.42*width]
    let end:Point
    if(n<=7)end=[frontHeight,front,4]
    else if(n<=10)end=[seventh[3]-(n-7)*1.25,seventh[4]+(n-7)*1.7,4+6*(n-7)]
    else end=point(1,n===11?.73:.48,n===11?.73:.91)
    let path:Point[]
    if(n>=11)path=[head,angle,point(.25,.015,.77),flank,end]
    else if(scale===500)path=[head,angle,flank,anterior,end]
    else if(scale===200||scale===100)path=[head,angle,flank,anterior,cartilage,end]
    else path=[head,point(.02,-.04,.19),angle,point(.27,.005,.78),flank,anterior,cartilage,end]
    const radius=scale===50?(figure==='tour-guide'?5.4:5.1):scale===100?7.5:4.5
    for(const [side,sign] of [['l',1],['r',-1]] as const)
      solids.push(tube('costal-'+n+'-'+side,path.map(([x,y,z])=>[x,y,z*sign]),radius,scale>=200))
  }
  if(scale===500)solids.push(tube('sternum',[rows[0],rows[3],seventh].map(r=>[r[3],r[4],0]),3.7,true))
  return solids
}
