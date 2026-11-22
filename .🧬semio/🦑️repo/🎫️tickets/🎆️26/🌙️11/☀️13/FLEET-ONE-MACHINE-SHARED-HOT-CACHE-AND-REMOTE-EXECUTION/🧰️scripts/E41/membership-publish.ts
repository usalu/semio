import {discoverCargoWorkspaces,publishSelectedCargoWorkspaceMembershipV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";
const root="/Users/ueli/Documents/semio",owners=discoverCargoWorkspaces(root);let stale=0;
for(const owner of owners){try{if(publishSelectedCargoWorkspaceMembershipV1(root,owner,(process.argv[2] as "check"|"write"),owners)){stale++;console.log("changed",owner.directory);}}catch(e){stale++;console.log("STALE",owner.directory,String((e as Error).message).slice(0,60));}}
console.log("owners",owners.length,"stale",stale);
