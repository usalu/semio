/** 📝️ Portable UTF8 fragment keeps every admitted write within the native stack extent. */
export class TextFragment{
 private readonly bytes=new Uint8Array(8192);private length=0;private poisoned=false;
 constructor(private readonly limit:number){if(!Number.isSafeInteger(limit)||limit<0||limit>8192)throw Error('layout-output-text-capacity');}
 write(value:string):boolean{if(this.poisoned)return false;let count=0;for(const scalar of value){const code=scalar.codePointAt(0)!;if(code>=0xd800&&code<=0xdfff){this.poisoned=true;return false;}count+=code<0x80?1:code<0x800?2:code<0x10000?3:4;if(count>this.limit-this.length){this.poisoned=true;return false;}}for(const scalar of value){const code=scalar.codePointAt(0)!;if(code<0x80)this.bytes[this.length++]=code;else if(code<0x800){this.bytes[this.length++]=0xc0|(code>>>6);this.bytes[this.length++]=0x80|(code&63);}else if(code<0x10000){this.bytes[this.length++]=0xe0|(code>>>12);this.bytes[this.length++]=0x80|((code>>>6)&63);this.bytes[this.length++]=0x80|(code&63);}else{this.bytes[this.length++]=0xf0|(code>>>18);this.bytes[this.length++]=0x80|((code>>>12)&63);this.bytes[this.length++]=0x80|((code>>>6)&63);this.bytes[this.length++]=0x80|(code&63);}}return true;}
 accepted():boolean{return!this.poisoned;}
 written():Uint8Array{return this.bytes.subarray(0,this.length);}
 finish():Uint8Array{if(this.poisoned)throw Error('layout-output-text-overflow');return this.written();}
}
