# BIM Berlin — Research Note for AP–Erfahrung
## Focused source analysis for *Entwerfen mit Bestand*

Tags: #digital-platform #urban-mining #material-supply #public-sector #reuse #interview-source #priority-high

---

## Why BIM Berlin matters for this research

From the perspective of **Entwerfen mit Bestand**, BIM Berlin is one of the most relevant external cases because it sits exactly at the intersection between:

- **public building stock**
- **reuse-oriented recovery and redistribution of components**
- **digital listing of components**
- and **pilot implementation in real projects**

That makes BIM Berlin valuable for all three research layers of the project:

1. **AP–Erfahrung**  
   understanding real workflows, data gaps, and bottlenecks

2. **AP–Plattform**  
   understanding how external catalogues and public actors could connect to a future open platform

3. **AP–Tool**  
   understanding which component data is missing when moving from listing to design, LCA, and structural plausibility

In short: **BIM Berlin is not just a marketplace example**. It is a strong case for studying how reused building components move from **public stock → recovery → listing → pilot reuse**, and where the digital chain is still incomplete.

---

# 1. Best BIM Berlin examples for the project

## 1.1 Bauteilauktion
**Link**  
https://www.bim-berlin.de/bauteilauktion

**What it is**  
A public-facing reuse marketplace for components from BIM-managed properties.

**Why it is highly relevant**  
This is the clearest BIM Berlin example for the project because it already performs one of the core functions your research is interested in:  
it makes **reused building components visible and transferable** through a digital interface.

**Contribution to the research**
- shows how components are currently presented to external users
- shows what a public-sector component marketplace looks like in practice
- helps identify the gap between **marketplace-ready data** and **design-ready data**
- useful reference for **API thinking**, catalogue integration, and metadata minimums

**Main research use**
- AP–Erfahrung: real workflow and bottlenecks
- AP–Plattform: external catalogue logic
- AP–Tool: what is still missing for design integration

**Connected examples**
- [[bim-berlin-data-model]]
- [[bim-berlin-reuse-pilot-friedrichshagen]]
- [[bim-berlin-reuse-concept-news]]

---

## 1.2 Bauteildetailseite / Datenmodell
**Link**  
https://www.bim-berlin.de/bauteilauktion/bauteile-details

**What it is**  
A concrete listing structure for individual components.

**Why it is highly relevant**  
This is one of the most useful BIM Berlin sources for your project because it exposes the **actual component fields** currently visible on the platform.

Visible fields include for example:
- dimensions
- weight
- material
- surface
- manufacturer
- year
- condition
- quantity
- availability
- collection / shipping
- note on hazardous substances
- provider

**Contribution to the research**
- helps define which fields already exist in practice
- makes it possible to compare **existing listing metadata** with the metadata your own platform would need
- reveals what is still missing for:
  - early design use
  - structural plausibility
  - automated LCA
  - compatibility checking

**Main research use**
- direct input for the **component schema**
- good basis for interview questions about data quality, effort, and missing fields

**Connected examples**
- [[bim-berlin-bauteilauktion]]
- [[madaster]]
- [[concular]]
- [[zrs-crclr-house]]

---

## 1.3 Re-Use-Pilot Wasserrettungsstation Friedrichshagen
**Link**  
https://www.bim-berlin.de/landesimmobilien/projekte-und-news/reuse-pilot-wasserrettungsstation-friedrichshagen

**What it is**  
A BIM Berlin pilot project in which reused components from BIM stock are integrated under a circular construction approach.

**Why it is highly relevant**  
This is important because it moves BIM Berlin beyond catalogue logic and into **actual project implementation**.  
For your research, that makes it especially useful: the project shows what happens when reused components are no longer only listed, but actually need to be **planned, verified, coordinated, and built**.

**Contribution to the research**
- reveals the bridge from stock and listing into project delivery
- useful for understanding how a reuse pilot is framed on the client / owner side
- helps identify what additional workflows become necessary once listed components enter real design and execution
- strong reference for interview questions about **selection, reservation, compatibility, and reuse strategy**

**Main research use**
- AP–Erfahrung: workflow mapping
- AP–Plattform: relation between stock owner and catalogue
- AP–Tool: what design-side support is needed when components enter a real project

**Connected examples**
- [[bim-berlin-bauteilauktion]]
- [[bim-berlin-spandauer-wuerfel]]
- [[bim-berlin-reuse-concept-news]]

---

## 1.4 Bauteilauktion / Re-Use concept news
**Links**  
https://www.bim-berlin.de/landesimmobilien/projekte-und-news/newsdetail/die-neue-bauteilauktion-ist-online  
https://www.bim-berlin.de/presse/details/neue-zukunft-fuer-alte-bauteile

**What it is**  
News and communication material framing the Bauteilauktion as part of a broader BIM reuse strategy.

**Why it is relevant**  
These texts are useful because they make BIM Berlin’s **institutional framing** explicit.  
They help clarify how BIM describes the reuse concept publicly: as climate protection, waste reduction, longer material life, and circular construction in practice.

**Contribution to the research**
- useful for understanding BIM’s stated objectives and vocabulary
- helps distinguish between **strategic reuse narrative** and **operational implementation**
- useful background for interviews: what BIM says publicly vs. what the workflow actually looks like

**Main research use**
- good preparation material before interviews
- useful for identifying where public communication may simplify real process complexity

**Connected examples**
- [[bim-berlin-bauteilauktion]]
- [[bim-berlin-reuse-pilot-friedrichshagen]]

---

## 1.5 Spandauer Würfel
**Links**  
https://www.bim-berlin.de/landesimmobilien/projekte-und-news/newsdetail/pilotprojekt-spandauer-wuerfel-neue-klassenraeume-im-re-use-verfahren  
https://www.bim-berlin.de/presse/details/pilotprojekt-spandauer-wuerfel-fertiggestellt-neue-klassenraeume-fuer-die-schule-an-der-havelduene

**What it is**  
A reuse project in which former residential containers were transformed into classroom space.

**Why it is relevant**  
This case is useful because it shows a different reuse model than component auctioning: **reconfiguration of existing built modules**.

**Contribution to the research**
- expands the BIM Berlin picture beyond single components
- useful for understanding reuse at **system or module scale**
- helps compare **component-level reuse** with **module-level reuse**
- may generate useful questions about what metadata changes when the reusable unit is no longer one element but a larger assembly

**Main research use**
- broadens the project’s case landscape
- useful as a comparative BIM Berlin example, not the primary one

**Connected examples**
- [[bim-berlin-reuse-pilot-friedrichshagen]]
- [[bim-berlin-bauteilauktion]]

---

## 1.6 Auction form / transaction logic
**Links**  
https://www.bim-berlin.de/bauteilauktion/bauteile-details/bauteilauktion-formular  
https://berichte.bim-berlin.de/magazin/detailseite/die-berliner-mischung-erhalten

**What it is**  
The transaction and legal side of the Bauteilauktion.

**Why it is relevant**  
This is not the most important design source, but it is useful because it shows the marketplace as a **transactional system**, with provider, bidder, legal responsibility, and limited warranty context.

**Contribution to the research**
- clarifies where responsibility sits in the current model
- useful for understanding why listing data may remain limited
- relevant for questions around trust, liability, and data completeness

**Main research use**
- background for interviews, especially around liability and data quality
- useful when comparing platform ambition with current market practice

**Connected examples**
- [[bim-berlin-bauteilauktion]]
- [[bim-berlin-data-model]]

---

# 2. Why BIM Berlin is especially strong from the project’s perspective

Based on the research project, BIM Berlin is strong because it touches several of the project’s core questions at once.

## 2.1 It helps with the metadata question
Your project asks which **component metadata** are required so reused elements can be used in design and performance assessment.  
BIM Berlin already exposes a basic metadata structure, which makes it a good real-world benchmark.

## 2.2 It helps with the workflow question
The project is interested in how reused components move through practice.  
BIM Berlin offers visible steps of this chain:
- public stock
- extraction / recovery
- listing
- selection / bidding
- pilot reuse

## 2.3 It helps with the platform question
Your project aims to connect external catalogues and create an open platform.  
BIM Berlin is therefore highly relevant as a **potential external actor type** the future platform would need to connect to.

## 2.4 It helps with the design-tool question
The Bauteilauktion is useful precisely because it is **not yet a design tool**.  
That gap is valuable: it helps identify what must be added so listed components can actually support:
- component matching
- design exploration
- structural plausibility
- LCA
- compatibility logic

---

# 3. Key gaps to investigate through BIM Berlin

These are the most important open questions for your research.

## 3.1 Metadata gap
Where does current marketplace metadata stop being useful for planning?

Likely gaps:
- no geometry-rich model
- no direct BIM / CAD integration
- no standardized structural properties
- no consistent LCA-ready data
- limited compatibility information

## 3.2 Workflow gap
What happens between:
- identifying a reusable element,
- publishing it,
- and actually using it in a design project?

This is probably where many of the hidden bottlenecks sit.

## 3.3 Verification gap
How is trust established?
- condition
- damage
- hazardous substances
- provenance
- technical fitness
- structural reliability

## 3.4 Interface gap
What would BIM Berlin need in order to connect to a future open platform?
- export structure
- API
- field mapping
- reservation status
- identity logic for components

## 3.5 Scale gap
How does reuse change when moving from:
- single listed components
to
- assemblies
to
- whole pilot projects

---

# 4. Suggested interview angles

## 4.1 Interview targets inside or around BIM Berlin
Depending on access, useful interview roles would be:

- people responsible for the **Bauteilauktion**
- people involved in **Reuse pilot projects**
- people responsible for **public stock management**
- communication / strategy staff who can explain BIM’s reuse framing
- project-side actors who dealt with real implementation

---

# 5. Interview questions
## Based on the project objectives and the BIM Berlin examples

### A. Component data and metadata
1. Which data fields are currently collected for components before they are listed?
2. Which of these fields are easy to obtain, and which are difficult or unreliable?
3. Which important component properties are currently not represented in the Bauteilauktion?
4. How do you deal with uncertainty in condition, dimensions, material, or age?
5. If components were to be used directly in design software, which extra fields would be needed?

### B. Workflow and process
6. What is the actual workflow from identifying a reusable component in stock to publishing it on the platform?
7. At which steps do the biggest delays or losses of information occur?
8. Who is responsible for collecting, verifying, and updating the component data?
9. How does the workflow change when a component is only sold versus actually reused in a BIM-led pilot project?
10. What happens after a component is selected by a buyer or project team?

### C. Design integration
11. What information do architects or planners usually ask for that is not available on the listing?
12. Have you seen cases where available component data was sufficient for direct planning use?
13. What would need to change for listed components to become design-ready rather than just market-ready?
14. Do you see a need for geometry models, simplified 3D representations, or BIM-compatible exports?
15. How should reservation, selection, and availability be communicated if components are used inside design workflows?

### D. Structural and technical verification
16. How are condition, damage, or hazardous substances currently documented?
17. Is there any structured process for technical evaluation beyond visual description?
18. Which component categories are easier to trust and reuse than others?
19. Where do structural or safety-related uncertainties become a barrier?
20. What kind of external verification would be needed for load-bearing reuse?

### E. Platform connection and future development
21. Could the current system be connected to another platform through export or API logic?
22. How standardized is the current data internally?
23. Which fields would you want to preserve if BIM Berlin were connected to a broader reuse platform?
24. What would make an external reuse platform genuinely useful for BIM Berlin?
25. Where do you see the strongest potential for automation: data entry, matching, verification, LCA, or logistics?

### F. Pilot-project perspective
26. In the Wasserrettungsstation pilot, what additional coordination steps were necessary compared with a normal project?
27. How were reusable components selected for actual project use?
28. What types of conflicts appeared between availability, timing, design needs, and verification?
29. What did the pilot reveal that would not be visible from the marketplace alone?
30. Which digital support would have saved the most time during the pilot?

---

# 6. Suggested outputs for AP–Erfahrung from a BIM Berlin interview

A good BIM Berlin interview could produce the following outputs for the project:

## For AP–Erfahrung
- workflow map from stock to listing to reuse
- list of current metadata fields
- list of missing metadata fields
- bottlenecks in public-sector reuse process
- actor map for responsibility and verification

## For AP–Plattform
- external catalogue requirements
- field-mapping logic for future integration
- API / export needs
- reservation and availability logic
- provenance and identity model for reused components

## For AP–Tool
- design-side data requirements
- what is needed for simplified structural plausibility
- what is needed for LCA linkage
- where AI assistance could help with matching, completion, or suggestions

---

# 7. Immediate research takeaways

If you are short on time, these are the most important reasons to prioritize BIM Berlin:

- it is a **real and current public-sector reuse case**
- it combines **stock ownership**, **digital listing**, and **pilot reuse**
- it gives you a visible starting point for discussing **component metadata**
- it helps expose the gap between **reuse marketplace** and **reuse design environment**
- it is highly aligned with your project’s goal of connecting catalogue, tool, and evaluation

---

# 8. Recommended companion sources to read next

To contextualize BIM Berlin properly, pair it with:

- **Concular** → for urban mining + digital supply logic  
  https://concular.de/

- **Madaster** → for long-term material identity and passport logic  
  https://madaster.com/

- **ZRS / CRCLR House** → for design + engineering use of reused elements  
  https://www.zrs.berlin/project/crclr-house/

- **B(e) Ware / Natural Building Lab** → for structural reuse and assessment  
  https://www.nbl.berlin/projects/reallabor-be-ware/

---

# 9. Suggested filename for the knowledge base

`bim-berlin-research-note.md`
