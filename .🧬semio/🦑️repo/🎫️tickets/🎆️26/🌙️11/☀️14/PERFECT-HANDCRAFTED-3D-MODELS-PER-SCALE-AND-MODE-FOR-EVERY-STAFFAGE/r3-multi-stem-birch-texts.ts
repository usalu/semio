/** 📝️ Decisions of the round-3 multi-stem birch, English first, German second: what each scale's volume models (`DECISIONS`) and what each scale's drawing shows (`DRAWN`).
 * @see ./r3-vegetation-multi-stem-birch.md */
type Text = { en: string; de: string };

const THERMAL: Text = {
  en: "Thermal: stem-bark is the stool with its roots, the dark stem bases, the white stems and the leaders by reference; branch-bark the six boned branches with their tips and the seven strongest laterals (base diameter from 36 mm) by reference, while the thinner laterals are twigs of the crown and carry no bark region; leaf-crown every sleeve and tress by reference (exchange area, coverage only). The transpiration emitter is a flat outlet lens on the upper side of sleeves, never a disc in the air.",
  de: "Thermik: stem-bark sind Stock mit Wurzelanläufen, dunkle Stammfüße, weiße Stämme und Leittriebe per Verweis; branch-bark die sechs Äste an Knochen mit ihren Spitzen und die sieben stärksten Seitenäste (Basisdurchmesser ab 36 mm) per Verweis, während die dünneren Seitenäste als Zweige zur Krone gehören und keine Rindenregion tragen; leaf-crown jede Laubhülle und jeder Hängestrang per Verweis (Austauschfläche, nur Abdeckung). Der Transpirationsemitter ist eine flache Austrittslinse auf der Oberseite von Laubhüllen, nie eine Scheibe in der Luft.",
};
const STRUCTURE: Text = {
  en: "Structure: the real load path of a clump birch: the stool as one block on its fixed support plate, each stem as wood under the bark along its own curved axis in two members and a leader, every branch as a tapering wood member that leaves its stem at the real joint (at 1:100 and 1:200 the boned branches and the seven scaffold laterals, without the twig-thin ones), joints at the rig joints, wood masses at the centres of mass on the stem and branch axes and one foliage mass in the sleeve of each foliage bone. No foliage and no bark is part of it.",
  de: "Tragwerk: der wirkliche Lastpfad einer mehrstämmigen Birke: der Stock als ein Block auf seiner eingespannten Auflagerplatte, jeder Stamm als Holz unter der Rinde entlang seiner eigenen gekrümmten Achse in zwei Stäben und einem Leittrieb, jeder Ast als sich verjüngender Holzstab, der seinen Stamm am wirklichen Gelenk verlässt (bei 1:100 und 1:200 die Äste an Knochen und die sieben Gerüstäste, ohne die zweigdünnen), Gelenke an den Gelenken des Rigs, Holzmassen in den Schwerpunkten auf den Stamm- und Astachsen und eine Laubmasse in der Laubhülle jedes Laubknochens. Laub und Rinde gehören nicht dazu.",
};

export const DECISIONS: Record<number, Text[]> = {
  50: [
    {
      en: "Body at 1:50, modelled part by part: a stool that flares to the ground with five surface roots; three slender stems that leave it side by side and splay as a tripod (left and right lean forward, the centre stem back), each a gently sinuous tapering loft with a dark, thicker, fissured base and white bark above, ending in a thin leader; six branches on bones and forty smaller branches round the stems, all ascending at 50 to 60 degrees and arching outwards; on every branch a leafy sleeve of short shoots, one or two side shoots with their own sleeves, and four to six pendulous tresses that leave the branch sideways, turn down and hang plumb, the outer ones longest; bare whip ends below some tresses; dark lenticel bands, diamond fissures above the bases and the black scar under every branch on the white bark.",
      de: "Körper bei 1:50, Teil für Teil modelliert: ein Stock, der zum Boden hin ausläuft, mit fünf Wurzelanläufen; drei schlanke Stämme, die ihn nebeneinander verlassen und sich als Dreibein spreizen (linker und rechter Stamm neigen sich nach vorn, der mittlere nach hinten), jeder ein leicht geschwungener, sich verjüngender Loft mit dunklem, dickerem, rissigem Fuß und weißer Rinde darüber, der in einem dünnen Leittrieb endet; sechs Äste an Knochen und vierzig kleinere Äste rings um die Stämme, alle mit 50 bis 60 Grad aufsteigend und nach außen überbogen; an jedem Ast eine Laubhülle aus Kurztrieben, ein oder zwei Seitentriebe mit eigener Hülle und vier bis sechs Hängestränge, die den Ast seitlich verlassen, nach unten drehen und lotrecht hängen, die äußeren am längsten; kahle Rutenenden unter einigen Strängen; dunkle Lentizellenbänder, Rautenrisse über den Stammfüßen und die schwarze Narbe unter jedem Ast auf der weißen Rinde.",
    },
    THERMAL,
    STRUCTURE,
  ],
  100: [
    {
      en: "Body at 1:100, authored anew: the same stool and roots on plainer sections, the stems on fewer stations, the same branches on four sections; every sleeve, side shoot and tress of 1:50 stays, because a tress is 3 mm wide on paper at this scale: the sleeves on fewer sections, each tress as a plain tapering capsule from below its turn to its hanging end; whip ends, fissures and two thirds of the lenticel bands are left out, the branch scars and nine broader bands stay as bark texture.",
      de: "Körper bei 1:100, neu modelliert: derselbe Stock mit Wurzelanläufen in schlichteren Querschnitten, die Stämme mit weniger Stützstellen, dieselben Äste mit vier Querschnitten; jede Laubhülle, jeder Seitentrieb und jeder Hängestrang von 1:50 bleibt, weil ein Strang bei diesem Maßstab auf dem Papier 3 mm breit ist: die Hüllen mit weniger Querschnitten, jeder Strang als schlichte, sich verjüngende Kapsel von unterhalb seiner Umbiegung bis zu seinem hängenden Ende; Rutenenden, Risse und zwei Drittel der Lentizellenbänder entfallen, die Astnarben und neun breitere Bänder bleiben als Rindentextur.",
    },
    THERMAL,
    STRUCTURE,
  ],
  200: [
    {
      en: "Body at 1:200, authored anew: stool without roots, stems with dark bases, leaders and every branch as a slender tapering loft on three sections; every sleeve and side shoot stays on five or six sections and every tress as a plain tapering capsule (1.5 mm wide on paper); no bark texture.",
      de: "Körper bei 1:200, neu modelliert: Stock ohne Wurzelanläufe, Stämme mit dunklen Füßen, Leittriebe und jeder Ast als schlanker, sich verjüngender Loft mit drei Querschnitten; jede Laubhülle und jeder Seitentrieb bleibt mit fünf oder sechs Querschnitten und jeder Hängestrang als schlichte, sich verjüngende Kapsel (auf dem Papier 1,5 mm breit); keine Rindentextur.",
    },
    THERMAL,
    STRUCTURE,
  ],
  500: [
    {
      en: "Body at 1:500, authored anew: the stool, three stems as plain tapering lofts of two sections per bone and their leaders; the crown as one weeping bough per branch and per upturned branch tip, each a single mass that follows the outer branch and turns over to hang, with a slim spire and one tress round each leader, so that the weeping outline and the open, airy crown with the stems showing still read; the branches themselves are not in the body.",
      de: "Körper bei 1:500, neu modelliert: der Stock, drei Stämme als schlichte, sich verjüngende Lofts mit zwei Querschnitten je Knochen und ihre Leittriebe; die Krone als eine hängende Zweigpartie je Ast und je aufgerichteter Astspitze, jeweils eine einzige Masse, die dem äußeren Ast folgt, umbiegt und herabhängt, mit einer schlanken Spitze und einem Strang um jeden Leittrieb, sodass der hängende Umriss und die offene, luftige Krone mit sichtbaren Stämmen lesbar bleiben; die Äste selbst gehören nicht zum Körper.",
    },
    {
      en: "Thermal: stem-bark is the stool, the stems and the leaders by reference; branch-bark is the six boned branches and their tips as slender solids hidden in the foliage; leaf-crown every bough by reference (exchange area, coverage only); three outlet lenses of the transpiration emitter on the upper side of sleeves.",
      de: "Thermik: stem-bark sind Stock, Stämme und Leittriebe per Verweis; branch-bark sind die sechs Äste an Knochen und ihre Spitzen als schlanke, im Laub verborgene Körper; leaf-crown jede Zweigpartie per Verweis (Austauschfläche, nur Abdeckung); drei Austrittslinsen des Transpirationsemitters auf der Oberseite von Laubhüllen.",
    },
    {
      en: "Structure: the stool block on its fixed support plate, each stem as wood under the bark in two members and a leader, the six boned branches and their tips as slender members, masses at the centres of mass and one foliage mass per foliage bone; the model is minimal for a three-stemmed tree.",
      de: "Tragwerk: der Stockblock auf seiner eingespannten Auflagerplatte, jeder Stamm als Holz unter der Rinde in zwei Stäben und einem Leittrieb, die sechs Äste an Knochen und ihre Spitzen als schlanke Stäbe, Massen in den Schwerpunkten und eine Laubmasse je Laubknochen; das Modell ist für einen dreistämmigen Baum minimal.",
    },
  ],
};

export const DRAWN: Record<number, Record<string, Text>> = {
  50: {
    silhouette: {
      en: "At 1:50 (1 m is 20 mm) the multi-stem birch is drawn in full as the true view of its body in all three views: three white stems splay from one dark stool, ascending branches arch outwards and carry leafy sleeves from which pendulous tresses hang plumb, the crown is open so that stems and branches show between the boughs. The side elevation shows the stems fanned out, the front elevation shows them as a V (left and right stems in front, the centre stem behind), the plan shows the boughs radiating from the three stems.",
      de: "Bei 1:50 (1 m sind 20 mm) ist die mehrstämmige Birke vollständig als wahre Ansicht ihres Körpers in allen drei Ansichten gezeichnet: drei weiße Stämme spreizen sich aus einem dunklen Stock, aufsteigende Äste biegen sich nach außen und tragen Laubhüllen, aus denen Hängestränge lotrecht herabhängen; die Krone ist offen, sodass Stämme und Äste zwischen den Zweigpartien sichtbar sind. Die Seitenansicht zeigt die Stämme aufgefächert, die Vorderansicht als V (linker und rechter Stamm vorn, der mittlere hinten), der Grundriss die von den drei Stämmen ausstrahlenden Zweigpartien.",
    },
    retained: {
      en: "Retained: the stool with its surface roots, the dark stem bases, the white stems and leaders, every branch and branch tip as a filled outline at its real width; the foliage of every bone as one outline behind the wood and one in front of it; a strand line down every tress and leaf ticks on the foliage in front; lenticel dashes, fissures and branch scars on the stem faces that look at the viewer; the bare whip ends as fine strokes; in plan the three stem sections at breast height.",
      de: "Beibehalten: der Stock mit seinen Wurzelanläufen, die dunklen Stammfüße, die weißen Stämme und Leittriebe, jeder Ast und jede Astspitze als gefüllter Umriss in wirklicher Breite; das Laub jedes Knochens als ein Umriss hinter dem Holz und einer davor; eine Stranglinie in jedem Hängestrang und Blattstriche auf dem vorderen Laub; Lentizellenstriche, Risse und Astnarben auf den dem Betrachter zugewandten Stammseiten; die kahlen Rutenenden als feine Striche; im Grundriss die drei Stammquerschnitte in Brusthöhe.",
    },
    merged: {
      en: "The sleeves, side shoots and tresses of one bone are merged into the outline of that bone's foliage (in front of the wood and behind it); every branch keeps its own outline.",
      de: "Die Laubhüllen, Seitentriebe und Hängestränge eines Knochens sind im Umriss des Laubs dieses Knochens vereint (vor dem Holz und dahinter); jeder Ast behält seinen eigenen Umriss.",
    },
    omitted: {
      en: "Omitted: single leaves, buds and catkins, which are below 0.5 mm on paper; strand lines and leaf ticks on the foliage behind the wood; bark marks on the far side of the stems and in plan.",
      de: "Weggelassen: einzelne Blätter, Knospen und Kätzchen, die auf dem Papier unter 0,5 mm liegen; Stranglinien und Blattstriche auf dem Laub hinter dem Holz; Rindenzeichen auf der abgewandten Stammseite und im Grundriss.",
    },
    thinElements: {
      en: "Whip ends, leaf ticks and lenticel dashes are the finest strokes (0.13 mm); the thinnest branch tips are drawn at their real width of 10 mm (0.2 mm on paper).",
      de: "Rutenenden, Blattstriche und Lentizellenstriche sind die feinsten Striche (0,13 mm); die dünnsten Astspitzen sind in ihrer wirklichen Breite von 10 mm gezeichnet (0,2 mm auf dem Papier).",
    },
    overlays: {
      en: "Each thermal region has its own shapes: stem-bark (stool, bases, stems, leaders), branch-bark (branches and tips) and leaf-crown (foliage per bone); stroke details carry no region and follow their bone.",
      de: "Jede thermische Region hat eigene Formen: stem-bark (Stock, Stammfüße, Stämme, Leittriebe), branch-bark (Äste und Spitzen) und leaf-crown (Laub je Knochen); Strichdetails tragen keine Region und folgen ihrem Knochen.",
    },
  },
  100: {
    silhouette: {
      en: "At 1:100 (1 m is 10 mm) the birch is the true view of its 1:100 body: the same three splayed white stems, arching branches and hanging tresses on simpler sections; the crown stays open and the stems show.",
      de: "Bei 1:100 (1 m sind 10 mm) ist die Birke die wahre Ansicht ihres 1:100-Körpers: dieselben drei gespreizten weißen Stämme, überbogenen Äste und Hängestränge mit einfacheren Querschnitten; die Krone bleibt offen und die Stämme sind sichtbar.",
    },
    retained: {
      en: "Retained: stool, dark stem bases, stems, leaders, branches and tips as filled outlines; the foliage of every bone behind and in front of the wood; one strand line per tress in front; branch scars and a few lenticel dashes; the stem sections in plan.",
      de: "Beibehalten: Stock, dunkle Stammfüße, Stämme, Leittriebe, Äste und Spitzen als gefüllte Umrisse; das Laub jedes Knochens hinter und vor dem Holz; eine Stranglinie je vorderem Hängestrang; Astnarben und wenige Lentizellenstriche; die Stammquerschnitte im Grundriss.",
    },
    merged: {
      en: "Sleeves, side shoots and tresses of one bone are merged into that bone's foliage outline, behind the wood and in front of it.",
      de: "Laubhüllen, Seitentriebe und Stränge eines Knochens sind zum Laubumriss dieses Knochens vereint, hinter dem Holz und davor.",
    },
    omitted: {
      en: "Omitted: leaf ticks, whip ends, fissures and most lenticel dashes; single leaves.",
      de: "Weggelassen: Blattstriche, Rutenenden, Risse und die meisten Lentizellenstriche; einzelne Blätter.",
    },
    thinElements: {
      en: "Thin strokes are at least 0.1 mm; branches are drawn at their real width, the thinnest tips are 0.1 mm on paper.",
      de: "Dünne Striche sind mindestens 0,1 mm; Äste sind in wirklicher Breite gezeichnet, die dünnsten Spitzen sind 0,1 mm auf dem Papier.",
    },
    overlays: {
      en: "Every region keeps its own shapes: stem-bark, branch-bark and leaf-crown per bone; strokes carry no region.",
      de: "Jede Region behält eigene Formen: stem-bark, branch-bark und leaf-crown je Knochen; Striche tragen keine Region.",
    },
  },
  200: {
    silhouette: {
      en: "At 1:200 (1 m is 5 mm) the birch is the true view of its 1:200 body: three white stems from a dark stool and an open weeping crown of boughs, each a sleeve with its hanging tresses.",
      de: "Bei 1:200 (1 m sind 5 mm) ist die Birke die wahre Ansicht ihres 1:200-Körpers: drei weiße Stämme aus einem dunklen Stock und eine offene, hängende Krone aus Zweigpartien, je eine Laubhülle mit ihren Hängesträngen.",
    },
    retained: {
      en: "Retained: stool, dark bases, stems, leaders and every branch as filled outlines at their real width; the foliage of every bone behind and in front of the wood; the stem sections in plan.",
      de: "Beibehalten: Stock, dunkle Stammfüße, Stämme, Leittriebe und jeder Ast als gefüllte Umrisse in wirklicher Breite; das Laub jedes Knochens hinter und vor dem Holz; die Stammquerschnitte im Grundriss.",
    },
    merged: {
      en: "All of a bone's foliage is merged into its outline, behind the wood and in front of it.",
      de: "Das gesamte Laub eines Knochens ist zu seinem Umriss vereint, hinter dem Holz und davor.",
    },
    omitted: {
      en: "Omitted: the surface roots, every strand line, leaf tick and bark mark.",
      de: "Weggelassen: die Wurzelanläufe, alle Stranglinien, Blattstriche und Rindenzeichen.",
    },
    thinElements: {
      en: "Slender branches are drawn as tapered filled outlines at their real section width and are not thickened.",
      de: "Schlanke Äste sind als sich verjüngende gefüllte Umrisse in wirklicher Querschnittsbreite gezeichnet und werden nicht verdickt.",
    },
    overlays: {
      en: "Every thermal region keeps its own shapes: stem-bark, branch-bark and leaf-crown per bone, so sway and loads act on the drawn shapes.",
      de: "Jede thermische Region behält eigene Formen: stem-bark, branch-bark und leaf-crown je Knochen, sodass Schwingung und Lasten an den gezeichneten Formen wirken.",
    },
  },
  500: {
    silhouette: {
      en: "At 1:500 (1 m is 2 mm) only the overall form is drawn, as the true view of the 1:500 body: one open weeping crown outline with its larger gaps over three slim white stems that splay from the stool; front view with the stems as a V, plan as the lobed crown with its openings.",
      de: "Bei 1:500 (1 m sind 2 mm) wird nur die Gesamtform gezeichnet, als wahre Ansicht des 1:500-Körpers: ein offener, hängender Kronenumriss mit seinen größeren Lücken über drei schlanken weißen Stämmen, die sich aus dem Stock spreizen; Vorderansicht mit den Stämmen als V, Grundriss als gelappte Krone mit ihren Öffnungen.",
    },
    retained: {
      en: "Retained: the stool, the three stems and their leaders as one stem shape, and the crown as one outline with the gaps that are larger than 0.3 m.",
      de: "Beibehalten: der Stock, die drei Stämme und ihre Leittriebe als eine Stammform und die Krone als ein Umriss mit den Lücken, die größer als 0,3 m sind.",
    },
    merged: {
      en: "Merged into the crown path: all foliage and the branches inside it. All stems are one shape.",
      de: "Im Kronenpfad vereint: das gesamte Laub und die darin liegenden Äste. Alle Stämme sind eine Form.",
    },
    omitted: {
      en: "Omitted: single branches, strand lines, bark marks and gaps below 0.3 m; they are far below 0.1 mm line width at this scale.",
      de: "Weggelassen: einzelne Äste, Stranglinien, Rindenzeichen und Lücken unter 0,3 m; sie liegen bei diesem Maßstab weit unter 0,1 mm Strichbreite.",
    },
    thinElements: {
      en: "Thin parts are not thickened: the stems are drawn at their real width (0.2 to 0.4 mm on paper), the branches are merged into the crown.",
      de: "Dünne Teile werden nicht verdickt: Die Stämme sind in wirklicher Breite gezeichnet (0,2 bis 0,4 mm auf dem Papier), die Äste gehen in der Krone auf.",
    },
    styles: {
      en: "Light line: closed outlines only. Dark solid: the crown path reads as one mass, the white stems lighter. Both stay readable at 36 mm printed height or less.",
      de: "Helle Linie: nur geschlossene Umrisse. Dunkle Fläche: Der Kronenpfad liest sich als eine Masse, die weißen Stämme heller. Beides bleibt bei höchstens 36 mm gedruckter Höhe lesbar.",
    },
    overlays: {
      en: "Thermal and structural overlays map onto stem-bark (the stem shape) and, merged in the crown path, leaf-crown and branch-bark. Merged paths show the area-weighted value of their regions, labelled as merged.",
      de: "Thermische und statische Überlagerungen bilden sich ab auf stem-bark (die Stammform) sowie, im Kronenpfad vereint, leaf-crown und branch-bark. Vereinte Pfade zeigen den flächengewichteten Wert ihrer Regionen und sind als vereint gekennzeichnet.",
    },
  },
};
