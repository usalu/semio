import { load, save, rep } from "./lib.ts";
const d = load("city-car");
const o = rep(d, 500).decisions.omitted;
o.en = o.en.replace("; steering wheel and headrests (0.17 m = 0.34 mm, below the 0.4 mm feature minimum of the line pens).", "; steering wheel (0.17 m = 0.34 mm) and front headrests (0.18 m wide but 0.09 m deep = 0.18 mm, below the 0.2 mm print minimum): the interior is a detail-pen element and this band draws the merged silhouette only.");
o.de = o.de.replace("; Lenkrad und Kopfstützen (0,17 m = 0,34 mm, unter dem Merkmalsminimum der Linienstifte von 0,4 mm).", "; Lenkrad (0,17 m = 0,34 mm) und Kopfstützen der Vordersitze (0,05 m tief = 0,1 mm, unter dem Druckminimum von 0,2 mm): der Innenraum ist ein Detailstift-Element und dieser Bereich zeichnet nur die zusammengefasste Silhouette.");
console.log(o.en); save("city-car", d);
