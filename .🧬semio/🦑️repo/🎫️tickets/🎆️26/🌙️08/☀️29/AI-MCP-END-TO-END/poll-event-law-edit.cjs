const fs=require('fs'),f='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/📥️input/🧪️tests/🦀️.rs',s=fs.readFileSync(f,'utf8'),x='            if ready&&row["wire"].get("Job").is_some(){',y=`            if ready&&row["wire"].get("Poll").is_some(){
                assert!(funded(&mut caller,grant,|cx|receiver.prepare_poll_event(cx)));
                assert!(!receiver.poll_event_pending());
                assert!(funded(&mut caller,grant,|cx|receiver.with_poll_step(cx,|poll,event,_|{assert!(poll.events.is_empty());assert!(event.is_none());Ok(())})).is_some());
            }
`+x;if(s.split(x).length!==2)throw Error('anchor');let p=f+'.poll-law';fs.writeFileSync(p,s.replace(x,y),{flag:'wx'});if(fs.readFileSync(f,'utf8')!==s){fs.unlinkSync(p);throw Error('concurrent change')}fs.renameSync(p,f);