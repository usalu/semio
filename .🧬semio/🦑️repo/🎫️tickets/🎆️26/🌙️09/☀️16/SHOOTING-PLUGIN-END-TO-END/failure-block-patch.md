# Failure block loses the stack of a long message (patch ready, NOT applied: identity file)

File: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏛️service/🧯️failure/🟨️.mjs`, function `failureBlockV1` (member of `serviceRevisionFiles`, so applying it changes the service identity).

Defect: the block is `head + detail` cut at 65536 bytes, and `detail` starts with the full message. A failure whose message is longer than the block (the test pads 90 000 characters) pushes every stack frame out of the block: the store keeps the message and none of the frames that say where it was thrown.

Patch: split `detail` at the first stack frame (`\n    at `); keep at most 16384 bytes of frames; cut the message part to what is left of the limit (`limit - head - frames`), marked with an ellipsis; behaviour for details without frames is unchanged.

```js
export function failureBlockV1({reference,detail,error,refusal,at=new Date(),limit=FAILURE_BLOCK_BYTES_V1}){
 let words;try{words=renderBothV1(refusal.key,refusal.fields);}catch{words={en:"",de:""};}
 const head=[...].join("\n"),frame=detail.indexOf("\n    at "),frames=frame<0?"":Buffer.from(detail.slice(frame)).subarray(0,FAILURE_FRAME_BYTES_V1).toString("utf8"),message=frame<0?detail:detail.slice(0,frame);
 const room=Math.max(0,limit-Buffer.byteLength(head)-Buffer.byteLength(frames)-3);
 const kept=Buffer.byteLength(message)<=room?message:Buffer.from(message).subarray(0,room).toString("utf8")+"...";
 return Buffer.from(head+kept+frames).subarray(0,limit).toString("utf8");
}
```
(`FAILURE_FRAME_BYTES_V1=16384` next to `FAILURE_BLOCK_BYTES_V1`.) The existing test `resident/tests/🌐️host` "real process: a resident service that throws ..." asserts exactly this (frames present, block between 60 000 and 65 536 bytes).
