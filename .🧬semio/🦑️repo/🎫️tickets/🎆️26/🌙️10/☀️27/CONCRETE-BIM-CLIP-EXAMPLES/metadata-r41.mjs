/** 📝 Apply verified bilingual provenance corrections without changing physical geometry. */
import {applyStairNotes} from './stair-r36-notes.mjs';
const notes={
 'piloti-diameter':{en:'Numbered supports use individually registered circular, rectangular, polygonal or hollow physical profiles and, where authored, level-specific sections. Source-supported dimensions and inferred envelopes are distinguished per element; no uniform30cm piloti diameter is assumed.',de:'Nummerierte Stützen verwenden einzeln registrierte kreisförmige, rechteckige, polygonale oder hohle physische Profile und gegebenenfalls höhenspezifische Abschnitte. Belegte Maße und abgeleitete Hüllen sind je Bauteil unterschieden; ein einheitlicher Pilotidurchmesser von30cm wird nicht angenommen.'},
 'infill-walls':{en:'Wall load-bearing status is assigned per element. Most above-ground partitions and facade infill are non-load-bearing; cellar masonry and the roof stair enclosure have explicit load-bearing assignments. The slab teaching analysis retains its selected load-path assumptions.',de:'Der tragende Status wird je Wand vergeben. Die meisten oberirdischen Trenn- und Fassadenausfachungen sind nichttragend; Kellermauerwerk und Dachtreppeneinfassung haben ausdrückliche tragende Zuordnungen. Die Decken-Lehranalyse behält ihren ausgewählten Lastabtrag mit benannten Annahmen.'},
 'tributary':{en:'Reactions are regional slab tributary-area estimates. A separate translation constructs shell and line-member geometry with assumed supports; stiffness remains unassessed and no finite-element system is solved.',de:'Auflagerkräfte sind regionale Decken-Lasteinzugsflächen-Näherungen. Eine separate Übersetzung erzeugt Schalen- und Stabgeometrie mit angenommenen Lagern; Steifigkeiten bleiben unbewertet und es wird kein Finite-Elemente-System gelöst.'},
 'ramp':{en:'Two return ramp assemblies use four straight flights between registeredY8.23m andY16.889m stations (8.659m per flight). Finished rises are3.17m ground to first and3.43m first to roof; gradients follow those endpoints. Flight widths, bearing details and the vertical interpretation of the sourced60mm slab thickness remain inferred.',de:'Zwei gegenläufige Rampenanlagen verwenden vier gerade Läufe zwischen registrierten StationenY8,23m undY16,889m (8,659m je Lauf). Fertighöhenunterschiede sind3,17m vom Erd- zum Obergeschoss und3,43m zum Dach; Neigungen folgen diesen Endpunkten. Laufbreiten, Lagerdetails und die vertikale Interpretation der belegten60mm Plattendicke bleiben abgeleitet.'}
};
export function applyMetadataNotes(model){
 applyStairNotes(model);
 for(const note of model.notes)if(notes[note.id])note.text=notes[note.id];
 for(const tag of ['ST-GF','ST-FR'])model.elements.find(e=>e.tag===tag).name={en:'Compensated U service stair',de:'Verzogene U-Diensttreppe'};
 model.elements.find(e=>e.tag==='ST-CG').name={en:'Compensated U cellar service stair',de:'Verzogene U-Kellertreppe'};
 const loss=model.translation.losses.find(l=>l.applies_to?.class==='IfcWall'&&l.property==='element');
 loss.en='Wall omitted from this slab teaching analysis; its physical load-bearing assignment is retained in the source.';
 loss.de='Wand in dieser Decken-Lehranalyse ausgelassen; ihre physische tragende Zuordnung bleibt in der Quelle erhalten.';
 const entry=model.notes.find(n=>n.id==='service-entry-placement');
 if(entry){entry.text.en=entry.text.en.replace('near the helical stair','near the compensated U stair');entry.text.de=entry.text.de.replace('Wendeltreppe','verzogenen U-Treppe');}
 return model;
}
