

const en200 = {
  silhouette: "Merged body silhouette of a skateboarder: head with beanie, hoodie, one form per sleeve with its fist, trousers per leg with the shoe, and the board as a deck on two wheels. Board and rider are drawn in all three views from the same bones, so the stance, the arms and the push cycle follow every pose and clip.",
  retained: "The crouched stance, the deck with kicked ends, both wheels at their real positions, and the arm with fist and the leg with shoe of each side apart.",
  merged: "Hood, hair and beanie are one head form; trousers and cuffs are one form per leg; truck and grip tape are merged into the deck.",
  omitted: "Face features, laces, soles, seams, trucks, toe lines, ears and the joint balls of elbow, knee and shoulder fall below the 0.2 mm minimum width at 1:200.",
  thinElements: "The 0.012 m deck is drawn as the thinnest printable plate and the 0.054 m wheels as 0.27 mm discs that read with the deck as one plate; body pen 0.22 mm.",
  hatching: "None; all parts are plain outlines.",
  styles: "Line and fill styles as at 1:100; every part keeps the region of its thermal and structural bindings.",
  overlays: "Thermal and structural overlays use the merged regions; the shoes and the hands share the trouser and sleeve regions where their own shapes are merged.",
};
const de200 = {
  silhouette: "Zusammengefasste Körperform einer Skateboardfahrerin: Kopf mit Mütze, Hoodie, je eine Form pro Ärmel mit Faust, Hose je Bein mit Schuh und das Brett als Deck auf zwei Rollen. Brett und Fahrer sind in allen drei Ansichten aus denselben Knochen gezeichnet, sodass Haltung, Arme und Schubbewegung jeder Pose und jedem Clip folgen.",
  retained: "Die geduckte Haltung, das Deck mit hochgezogenen Enden, beide Rollen an ihren wirklichen Orten sowie Arm mit Faust und Bein mit Schuh jeder Seite getrennt.",
  merged: "Kapuze, Haar und Mütze sind eine Kopfform; Hose und Säume sind eine Form pro Bein; Achse und Griptape gehen im Deck auf.",
  omitted: "Gesichtszüge, Schnürung, Sohlen, Nähte, Achsen, Zehenlinien, Ohren und die Gelenkkugeln von Ellbogen, Knie und Schulter unterschreiten bei 1:200 die Mindestbreite von 0,2 mm.",
  thinElements: "Das 0,012 m dicke Deck ist als dünnste druckbare Platte gezeichnet und die 0,054 m großen Rollen als 0,27-mm-Scheiben, die sich mit dem Deck als eine Platte lesen; Körperstift 0,22 mm.",
  hatching: "Keine; alle Teile sind einfache Umrisse.",
  styles: "Linien- und Flächenstile wie bei 1:100; jedes Teil behält die Region seiner thermischen und strukturellen Bindungen.",
  overlays: "Thermische und strukturelle Überlagerungen nutzen die zusammengefassten Regionen; Schuhe und Hände teilen die Hosen- und Ärmelregionen, wo ihre eigenen Formen zusammengefasst sind.",
};
const en500 = {
  silhouette: "A 3.5 mm tall site-plan mark of a skateboarder: head, body, one form per arm and leg and the board as a deck on two wheels, drawn in all three views from the same bones so the stance and the push cycle follow every pose and clip.",
  retained: "The crouched stance, the deck and its two wheels at their real positions, and the arms and legs of each side apart at the real heights of 1.77 m and the real board length.",
  merged: "Head, beanie and hood are one form; hoodie and sleeves, trousers and shoes and the hands are merged into the limb they end; truck and grip tape are part of the deck.",
  omitted: "Face, hair, laces, soles, cuffs, seams, fists, shoes as separate parts and all joint balls: they are below 0.2 mm at 1:500.",
  thinElements: "Limbs are enlarged to a minimum width of 0.105 m (0.21 mm) and the 0.012 m deck is drawn as the thinnest printable plate (pen 0.13 mm) with the wheels read as part of it.",
  hatching: "None.",
  styles: "Plain filled silhouettes in the body tone of each region.",
  overlays: "Thermal and structural overlays use the merged regions.",
};
const de500 = {
  silhouette: "Ein 3,5 mm hohes Lageplanzeichen einer Skateboardfahrerin: Kopf, Körper, je eine Form pro Arm und Bein und das Brett als Deck auf zwei Rollen, in allen drei Ansichten aus denselben Knochen gezeichnet, sodass Haltung und Schubbewegung jeder Pose und jedem Clip folgen.",
  retained: "Die geduckte Haltung, das Deck mit seinen zwei Rollen an ihren wirklichen Orten sowie Arme und Beine jeder Seite getrennt in den wirklichen Maßen von 1,77 m Höhe und der echten Brettlänge.",
  merged: "Kopf, Mütze und Kapuze sind eine Form; Hoodie und Ärmel, Hose und Schuhe sowie die Hände gehen in dem Glied auf, an dem sie enden; Achse und Griptape sind Teil des Decks.",
  omitted: "Gesicht, Haar, Schnürung, Sohlen, Säume, Nähte, Fäuste, Schuhe als eigene Teile und alle Gelenkkugeln: sie liegen bei 1:500 unter 0,2 mm.",
  thinElements: "Glieder sind auf eine Mindestbreite von 0,105 m (0,21 mm) vergrößert und das 0,012 m dicke Deck ist als dünnste druckbare Platte gezeichnet (Stift 0,13 mm), die Rollen lesen sich als Teil davon.",
  hatching: "Keine.",
  styles: "Einfache gefüllte Silhouetten im Körperton jeder Region.",
  overlays: "Thermische und strukturelle Überlagerungen nutzen die zusammengefassten Regionen.",
};

export const spec200 = { slug: "skateboarder", den: 200, drop: /^(eye|hood|hood-seam|deck-grip|ear-[lr]|hair|knee-[lr]|elbow-[lr]|shoulder-[lr]|truck-front|cuff-[lr]|sole.*|toe.*)$/, minFeatureM: 0.06, epsM: 0.005, pens: { body: 0.22, detail: null, texture: null, glazing: null, ground: 0.25 }, range: [150, 350], en: en200, de: de200 };
export const spec500 = { slug: "skateboarder", den: 500, drop: /^(eye|hood|hood-seam|deck-grip|ear-[lr]|hair|knee-[lr]|elbow-[lr]|shoulder-[lr]|truck-front|cuff-[lr]|sole.*|toe.*)$/, minFeatureM: 0.1, epsM: 0.012, pens: { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.18 }, range: [350, 900], en: en500, de: de500 };
