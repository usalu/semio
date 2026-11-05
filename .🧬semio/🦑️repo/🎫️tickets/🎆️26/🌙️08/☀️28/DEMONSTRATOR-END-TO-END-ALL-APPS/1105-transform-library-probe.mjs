try{const m=await import("three");console.log("existing third-party Quaternion="+(typeof m.Quaternion));}catch(error){console.log("existing three unavailable: "+error.code);}
