// 🎯 Maps the eight product tasks to their generated Bazel labels and prints each closure size from cquery output files.
import {createHash} from "node:crypto";
export const PRODUCTS=["@teaching/architecture-course:build","@teaching/architecture-course:test","@teaching/proctor:build","@teaching/proctor:test","@semio-tech/semio-tech-play:activate-prototype","@semio-tech/semio-tech-play:build","@semio-tech/mit-bestand-demonstrator:activate-prototype","@semio-tech/mit-bestand-demonstrator:build"];
export const labelOf=(id:string)=>"t_"+createHash("sha256").update(id).digest("hex").slice(0,16);
if(import.meta.main)for(const id of PRODUCTS)console.log(id+"\t//:"+labelOf(id));
