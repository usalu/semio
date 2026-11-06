import {readFileSync,writeFileSync} from 'node:fs';
const path='🧰️framework/🔨️modules/🌱️value/🛬️decode/🎟️work/🧪️tests/🦀️.rs';let body=readFileSync(path,'utf8');
const old='let mut control=NativeDecodeControl::new_forwarded(0,&mut observer,&mut allocation);control.install_retirement_recipient(&mut retirement).unwrap();let mut receipt=None;';
const next='let mut receipt=None;';if(!body.includes(old))throw new Error('original forwarded law missing');body=body.replace(old,next);
const oldFrame='let mut frame=NativeForwardedDecodeFrame::new(control,&mut receipt).ok().unwrap();';
const nextFrame='let mut frame=NativeForwardedDecodeFrame::begin(&mut receipt,0,&mut observer,&mut allocation,&mut retirement).unwrap();';if(!body.includes(oldFrame))throw new Error('original frame constructor missing');body=body.replace(oldFrame,nextFrame);writeFileSync(path,body);
