#!/usr/bin/env python3
"""🏛️ W2-R architect: rewrites every `s.architect.program` leaf payload schema to the truth of its Rust payload and
annotates every input with `x-semio-ui` (design §6, manifest `$defs/InputUi`).

Truth: a record payload is the snapshot `$defs` record (nullability, `required`, nesting) with enum values taken from the
Rust enums (`rename_all = "camelCase"`) and integer types taken from the Rust field types (`u32`/`u64` → integer ≥ 0);
`EntityId` inputs are strings. Labels are hand-written en/de terms of architectural programming (DIN 18205 Bedarfsplanung,
DIN 276/277, DIN 18040, HOAI phases); units come from the Rust `#[dsl(unit = …)]` attributes and the field names.

    python3 <ticket>/🧪️w2-r-architect-program.py
"""
import json
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"
SUBSET = REPO + "/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any"
SCHEMA = SUBSET + "/🧬️schema"
LEAVES = SCHEMA + "/🧬️mutations"
CONFIG_LEAF = SUBSET + "/✏️editor/🎚️config/🧬️schema/🧬️mutations/📸️replace-config/🧬️schema/🔣️.json"
PRESENCE_LEAF = SUBSET + "/✏️editor/👥️presence/🧬️schema/🧬️mutations/📸️replace-presence/🧬️schema/🔣️.json"
DOMAIN = "program"
GRANULARITY = "entity"
KEY_ORDER = ("widget", "role", "label", "description", "ref", "unit", "displayUnit", "displayFactor", "step", "precision", "softMin", "softMax", "scale", "snaps", "snapSource", "options", "group", "order")
UNSIGNED = {"u8", "u16", "u32", "u64", "usize"}
SIGNED = {"i8", "i16", "i32", "i64", "isize"}
FLOATS = {"f32", "f64"}


def L(en, de):
    return {"en": en, "de": de}


def ui(**body):
    return {key: body[key] for key in KEY_ORDER if key in body and body[key] is not None}


#region 🔖️Rust
def rust_source():
    return open(SCHEMA + "/🗄️registers/🦀️.rs", encoding="utf-8").read() + "\n" + open(SCHEMA + "/🧱️kernel/🦀️.rs", encoding="utf-8").read()


def rust_structs(source):
    structs = {}
    for match in re.finditer(r"pub struct (\w+) \{(.*?)\n\}", source, re.S):
        fields = {}
        unit = None
        for line in match.group(2).split("\n"):
            text = line.strip()
            found = re.match(r'#\[dsl\(unit = "([^"]+)"\)\]', text)
            if found:
                unit = found.group(1)
                continue
            field = re.match(r"pub (\w+): (.+),$", text)
            if field:
                camel = re.sub(r"_([a-z0-9])", lambda part: part.group(1).upper(), field.group(1))
                fields[camel] = {"type": field.group(2), "unit": unit}
                unit = None
        structs.setdefault(match.group(1), fields)
    return structs


def rust_enums(source):
    enums = {}
    for match in re.finditer(r"((?:#\[[^\n]*\]\s*\n)*)pub enum (\w+) \{(.*?)\n\}", source, re.S):
        if 'rename_all = "camelCase"' not in match.group(1):
            continue
        values = []
        for line in match.group(3).split("\n"):
            variant = re.match(r"(\w+)\s*,?$", line.strip())
            if variant:
                values.append(variant.group(1)[0].lower() + variant.group(1)[1:])
        enums[match.group(2)] = values
    return enums


SOURCE = rust_source()
STRUCTS = rust_structs(SOURCE)
ENUMS = rust_enums(SOURCE)
SNAP = json.load(open(SCHEMA + "/📸️snapshot/🔣️.json", encoding="utf-8"))["$defs"]


def base_type(text):
    option = vec = False
    while True:
        found = re.fullmatch(r"(Option|Vec|Box)<(.+)>", text)
        if not found:
            return text, option, vec
        option = option or found.group(1) == "Option"
        vec = vec or found.group(1) == "Vec"
        text = found.group(2)


def rust_field(struct, field):
    fields = STRUCTS[struct]
    if field in fields:
        return fields[field]
    if "header" in fields and field in STRUCTS["EntityHeader"]:
        return STRUCTS["EntityHeader"][field]
    raise SystemExit("🏛️ %s.%s has no Rust field" % (struct, field))
#endregion 🔖️Rust


#region 🔖️Truth
def truth(node, struct, field):
    if "$ref" in node:
        name = node["$ref"].rsplit("/", 1)[1]
        if "enum" in SNAP[name]:
            return {"type": "string", "enum": list(ENUMS[name])}
        return truth_object(name)
    if "anyOf" in node:
        return {"anyOf": [branch if branch == {"type": "null"} else truth(branch, struct, field) for branch in node["anyOf"]]}
    if node.get("type") == "array":
        out = {key: value for key, value in node.items() if key != "items"}
        out["items"] = truth(node["items"], struct, field)
        return out
    if node.get("type") in ("number", "integer"):
        base, _, _ = base_type(rust_field(struct, field)["type"])
        if base in UNSIGNED:
            return {"type": "integer", "minimum": 0}
        if base in SIGNED:
            return {"type": "integer"}
        if base in FLOATS:
            return {"type": "number"}
        raise SystemExit("🏛️ %s.%s is numeric in the snapshot but %s in Rust" % (struct, field, base))
    if "enum" in node:
        raise SystemExit("🏛️ %s.%s carries an inline enum" % (struct, field))
    return dict(node)


def truth_object(name):
    definition = SNAP[name]
    out = {"title": name, "type": "object", "additionalProperties": definition.get("additionalProperties", False)}
    if "required" in definition:
        out["required"] = sorted(definition["required"])
    out["properties"] = {key: truth(value, name, key) for key, value in definition["properties"].items()}
    return out
#endregion 🔖️Truth


#region 🔖️Vocabulary
ENTITIES = {
    "accessRule": ("Access Rule", "Zugangsregel"),
    "accessibilityRequirement": ("Accessibility Requirement", "Barrierefreiheitsanforderung"),
    "activity": ("Activity", "Aktivität"),
    "adjacency": ("Adjacency", "Adjazenz"),
    "analysisRecord": ("Analysis", "Analyse"),
    "approvalRecord": ("Approval", "Freigabe"),
    "assumption": ("Assumption", "Annahme"),
    "auditEvent": ("Audit Event", "Audit-Ereignis"),
    "benchmarkRecord": ("Benchmark", "Kennwert"),
    "changeRecord": ("Change Record", "Änderungseintrag"),
    "collaborationRecord": ("Collaboration Record", "Zusammenarbeitseintrag"),
    "communicationRequirement": ("Communication Requirement", "Kommunikationsanforderung"),
    "complianceRecord": ("Compliance Record", "Konformitätsnachweis"),
    "conflict": ("Conflict", "Konflikt"),
    "constraintRecord": ("Constraint", "Randbedingung"),
    "costRequirement": ("Cost Requirement", "Kostenanforderung"),
    "decision": ("Decision", "Entscheidung"),
    "deliveryConstraint": ("Delivery Constraint", "Abwicklungsrandbedingung"),
    "document": ("Document", "Dokument"),
    "environmentalRequirement": ("Environmental Requirement", "Umgebungsanforderung"),
    "equipment": ("Equipment", "Ausstattung"),
    "flexibilityRequirement": ("Flexibility Requirement", "Flexibilitätsanforderung"),
    "flowRequirement": ("Flow Requirement", "Flussanforderung"),
    "function": ("Function", "Funktion"),
    "governance": ("Governance", "Governance"),
    "growthPlan": ("Growth Plan", "Wachstumsplan"),
    "humanFactorRequirement": ("Human Factors Requirement", "Human-Factors-Anforderung"),
    "informationRequirement": ("Information Requirement", "Informationsanforderung"),
    "infrastructureRequirement": ("Infrastructure Requirement", "Infrastrukturanforderung"),
    "issue": ("Issue", "Offener Punkt"),
    "knowledgeRecord": ("Knowledge Record", "Wissenseintrag"),
    "meetingRecord": ("Meeting Record", "Besprechungsprotokoll"),
    "meta": ("Program Metadata", "Programm-Metadaten"),
    "operationalRequirement": ("Operational Requirement", "Betriebsanforderung"),
    "optionEvaluation": ("Option Evaluation", "Variantenbewertung"),
    "organizationalRequirement": ("Organisational Requirement", "Organisatorische Anforderung"),
    "performanceCriterion": ("Performance Criterion", "Leistungskriterium"),
    "priorityRecord": ("Priority Record", "Prioritätseintrag"),
    "privacyRequirement": ("Privacy Requirement", "Privatsphäreanforderung"),
    "process": ("Process", "Prozess"),
    "programElement": ("Program Element", "Programmelement"),
    "project": ("Project Definition", "Projektdefinition"),
    "qualityRecord": ("Quality Record", "Qualitätseintrag"),
    "quantityRequirement": ("Quantity Requirement", "Mengenanforderung"),
    "regulatoryRequirement": ("Regulatory Requirement", "Regulatorische Anforderung"),
    "relationship": ("Relationship", "Beziehung"),
    "reportRecord": ("Report", "Bericht"),
    "requirement": ("Requirement", "Anforderung"),
    "resilienceRequirement": ("Resilience Requirement", "Resilienzanforderung"),
    "resource": ("Resource", "Ressource"),
    "risk": ("Risk", "Risiko"),
    "safetyRequirement": ("Safety Requirement", "Sicherheitsanforderung"),
    "scenario": ("Scenario", "Szenario"),
    "scheduleRequirement": ("Schedule Requirement", "Terminanforderung"),
    "searchFilter": ("Search Filter", "Suchfilter"),
    "securityRequirement": ("Security Requirement", "Sicherungsanforderung"),
    "serviceRequirement": ("Service Requirement", "Dienstleistungsanforderung"),
    "siteContext": ("Site Context", "Standortkontext"),
    "stakeholder": ("Stakeholder", "Projektbeteiligter"),
    "statusRecord": ("Status Record", "Statuseintrag"),
    "storageRequirement": ("Storage Requirement", "Lageranforderung"),
    "survey": ("Survey", "Umfrage"),
    "sustainabilityRequirement": ("Sustainability Requirement", "Nachhaltigkeitsanforderung"),
    "templateRecord": ("Template", "Vorlage"),
    "trace": ("Trace Link", "Nachverfolgungsbeziehung"),
    "userProfile": ("User Profile", "Nutzerprofil"),
    "validationRecord": ("Validation Record", "Validierungseintrag"),
    "wayfindingRequirement": ("Wayfinding Requirement", "Leitsystemanforderung"),
    "workshop": ("Workshop", "Workshop"),
}
DEFINITIONS = {"document": "ArtifactRecord", "trace": "TraceLink", "meta": "ProgramMeta", "project": "ProjectDefinition", "governance": "Governance"}

LABEL_TABLE = """
abilities|Abilities|Fähigkeiten|Physical and cognitive abilities of this user group.|Körperliche und kognitive Fähigkeiten dieser Nutzergruppe.
acceptanceCriteria|Acceptance Criteria|Abnahmekriterien
accessControls|Access Controls|Zugriffskontrollen
accessFrequency|Access Frequency|Zugriffshäufigkeit|How often the stored goods are accessed.|Wie oft auf das Lagergut zugegriffen wird.
accessLevel|Access Level|Zugangsstufe
accessMode|Access Mode|Zugangsart
accessPath|Access Path|Zugangsweg|Route by which one element is reached from the other.|Weg, über den ein Element vom anderen aus erreicht wird.
accessRestrictions|Access Restrictions|Zugangsbeschränkungen
accessRoads|Access Roads|Zufahrtsstraßen
accessibility|Accessibility Provisions|Barrierefreie Vorkehrungen
accessibilityNeeds|Accessibility Needs|Anforderungen an die Barrierefreiheit
accountabilityRules|Accountability Rules|Verantwortlichkeitsregeln
acousticClass|Acoustic Class|Akustikklasse|Required acoustic class of the space, e.g. per DIN 4109 or DIN 18041.|Geforderte akustische Klasse des Raums, z. B. nach DIN 4109 oder DIN 18041.
acousticPrivacy|Acoustic Privacy|Akustische Vertraulichkeit
acousticTarget|Acoustic Target|Akustischer Zielwert
action|Action|Aktion
actionItems|Action Items|Aufgaben
activityIds|Activities|Aktivitäten
activityLinkIds|Activity Links|Aktivitätsverknüpfungen|Trace links that tie this equipment to activities.|Nachverfolgungsbeziehungen, die diese Ausstattung mit Aktivitäten verbinden.
activityType|Activity Type|Aktivitätsart
actorId|Actor|Akteur
actors|Actors|Akteure
adaptationScenarios|Adaptation Scenarios|Anpassungsszenarien
address|Address|Adresse
adjacencyIds|Adjacencies|Adjazenzen
adjacencyPreferences|Preferred Neighbours|Bevorzugte Nachbarelemente|Program elements this element should preferably adjoin.|Programmelemente, an die dieses Element bevorzugt angrenzen soll.
adjacentActivities|Adjacent Activities|Benachbarte Aktivitäten
affectedElementIds|Affected Elements|Betroffene Elemente
affectedEntityIds|Affected Entities|Betroffene Entitäten
affectedRequirementIds|Affected Requirements|Betroffene Anforderungen
affectedUserIds|Affected User Profiles|Betroffene Nutzerprofile
afterSnapshot|State After|Zustand danach|Serialised state after the change.|Serialisierter Zustand nach der Änderung.
afterState|State After|Zustand danach|Serialised state after the event.|Serialisierter Zustand nach dem Ereignis.
ageRange|Age Range|Altersspanne
agenda|Agenda|Tagesordnung
agendaItems|Agenda Items|Tagesordnungspunkte
allocation|Allocation|Zuteilung
alternateSites|Alternate Sites|Ausweichstandorte
amount|Amount|Betrag|Cost amount in the currency of this record.|Kostenbetrag in der Währung dieses Eintrags.
analysisId|Analysis|Analyse
analysisIds|Analyses|Analysen
applicability|Applicability|Anwendungsbereich
applicableElementKinds|Applicable Element Kinds|Anwendbare Elementarten
applicableSectors|Applicable Sectors|Anwendbare Branchen
approvalDate|Approval Date|Freigabedatum
approvalGates|Approval Gates|Freigabepunkte
approvalIds|Approvals|Freigaben
approvalMatrix|Approval Matrix|Freigabematrix
approvalStatus|Approval Status|Freigabestatus
approvalType|Approval Type|Freigabeart
approvedBy|Approved By|Freigegeben von
approverId|Approver|Freigebende Person
approverIds|Approvers|Freigebende
area|Floor Area|Fläche|Required floor area band; the unit is part of the quantity, e.g. m² per DIN 277.|Geforderte Flächenspanne; die Einheit gehört zur Mengenangabe, z. B. m² nach DIN 277.
areaDelta|Area Change (m²)|Flächenänderung (m²)|Change in floor area against the baseline.|Änderung der Fläche gegenüber der Basis.
areaGrowth|Area Growth|Flächenwachstum
artifactRefs|Documents|Dokumente
aspect|Aspect|Aspekt
assetIds|Protected Assets|Schutzgüter|Entities this security requirement protects.|Entitäten, die diese Sicherungsanforderung schützt.
assigneeId|Assignee|Bearbeitende Person
assumptions|Assumptions|Annahmen
attachments|Attachments|Anhänge
attendeeIds|Attendees|Teilnehmende
audience|Audience|Zielgruppe
audienceIds|Audience|Zielgruppe
audioRequired|Audible Guidance Required|Akustische Leitinformation erforderlich
auditDate|Audit Date|Auditdatum
auditEventIds|Audit Events|Audit-Ereignisse
auditRequired|Logging Required|Protokollierung erforderlich
auditRequirements|Audit Requirements|Protokollierungsanforderungen
auditSchedule|Audit Schedule|Auditplan
auditTrail|Audit Trail|Prüfpfad|Whether changes to this information are logged.|Ob Änderungen an dieser Information protokolliert werden.
auditTrailRef|Audit Trail Reference|Verweis auf den Prüfpfad
auditorId|Auditor|Auditor
auditoryDemands|Auditory Demands|Auditive Anforderungen
authentication|Authentication|Authentifizierung
authorId|Author|Autor
authorIds|Authors|Autoren
authority|Issuing Authority|Zuständige Behörde
authorityBasis|Authority Basis|Befugnisgrundlage
authorityId|Accountable|Entscheidungsbefugt|The stakeholder accountable for this entry.|Der Beteiligte, der für diesen Eintrag entscheidungsbefugt ist.
authorization|Authorisation|Autorisierung
automationLevel|Automation Level|Automatisierungsgrad
availability|Availability|Verfügbarkeit
average|Average|Mittelwert
backupRequirements|Backup Requirements|Datensicherungsanforderungen
backupService|Backup Services|Ausweichdienste
backupSystems|Backup Systems|Ersatzsysteme
badgeRequired|Badge Required|Ausweis erforderlich
barrierFree|Barrier-Free|Barrierefrei
baseline|Baseline|Ausgangswert
Scenario.baseline|Baseline Scenario|Basisszenario|Whether this scenario is the reference the others are compared with.|Ob dieses Szenario die Referenz für den Vergleich der anderen ist.
basis|Basis|Grundlage
CostRequirement.basis|Cost Basis|Kostenbasis
QuantityRequirement.basis|Basis|Bemessungsgrundlage
beforeSnapshot|State Before|Zustand davor|Serialised state before the change.|Serialisierter Zustand vor der Änderung.
beforeState|State Before|Zustand davor|Serialised state before the event.|Serialisierter Zustand vor dem Ereignis.
behavioralPatterns|Behavioural Patterns|Verhaltensmuster
benchmarkIds|Benchmarks|Kennwerte
benchmarkName|Benchmark Name|Kennwertbezeichnung
benchmarkRef|Benchmark|Kennwert
benefits|Benefits|Vorteile
bestPractices|Best Practices|Bewährte Verfahren
bidirectional|Bidirectional|Bidirektional
biodiversity|Biodiversity|Biodiversität
biometricRequired|Biometrics Required|Biometrie erforderlich
blockers|Blockers|Hindernisse
bottlenecks|Bottlenecks|Engpässe
brandIntegration|Brand Integration|Markenintegration
brandingRequirements|Branding Requirements|Anforderungen an das Corporate Design
breachResponse|Breach Response|Reaktion auf Datenschutzverletzungen
briefSummary|Brief Summary|Zusammenfassung des Bedarfsplans|Summary of the project brief (needs assessment per DIN 18205).|Zusammenfassung des Bedarfsplans nach DIN 18205.
budget|Budget|Budget
budgetEnvelope|Budget Envelope|Kostenrahmen|Upper budget limit of the growth plan.|Obergrenze des Budgets für den Wachstumsplan.
buildingType|Building Type|Gebäudetyp
calculationMethod|Calculation Method|Berechnungsmethode
capacity|Capacity|Kapazität
capacityConstraint|Capacity Constraint|Kapazitätsgrenze
cashFlowProfile|Cash Flow Profile|Mittelabflussprofil
categories|Categories|Kategorien
category|Category|Kategorie
UserProfile.category|User Category|Nutzerkategorie
causes|Causes|Ursachen
certification|Certifications|Zertifizierungen|Sustainability certifications sought, e.g. DGNB, BNB, LEED or BREEAM.|Angestrebte Nachhaltigkeitszertifizierungen, z. B. DGNB, BNB, LEED oder BREEAM.
certificationTarget|Certification Target|Zertifizierungsziel
certificationTargets|Certification Targets|Zertifizierungsziele
chairId|Chair|Sitzungsleitung
changeControlProcess|Change Control Process|Änderungsmanagementprozess
changeDate|Change Date|Änderungsdatum
changeReadiness|Change Readiness|Veränderungsbereitschaft
changeRecordId|Change Record|Änderungseintrag
changeType|Change Type|Änderungsart
changedAt|Changed At|Geändert am
changedBy|Changed By|Geändert von
channel|Channel|Kanal
charts|Charts|Diagramme
checklists|Checklists|Checklisten
checksum|Checksum|Prüfsumme
chemicalSafety|Hazardous Substances|Gefahrstoffschutz
childRequirementIds|Child Requirements|Untergeordnete Anforderungen
circulationOverlap|Circulation Overlap|Überlagerung der Verkehrswege|Whether the circulation of both elements overlaps.|Ob sich die Verkehrswege beider Elemente überlagern.
circulationRole|Circulation Role|Rolle in der Erschließung
citations|Citations|Quellenangaben
InformationRequirement.classification|Confidentiality Class|Vertraulichkeitsstufe
ArtifactRecord.classification|Confidentiality Class|Vertraulichkeitsstufe
ProgramMeta.classification|Classification Systems|Klassifikationssysteme|Classification systems the program follows, e.g. DIN 276, DIN 277 or Uniclass.|Klassifikationssysteme, denen das Programm folgt, z. B. DIN 276, DIN 277 oder Uniclass.
classifiedLevel|Classification Level|Geheimhaltungsgrad
clause|Clause|Abschnitt
cleaningRegime|Cleaning Regime|Reinigungskonzept
cleaningRequirements|Cleaning Requirements|Reinigungsanforderungen
clearHeightM|Clear Height (m)|Lichte Höhe (m)
clearWidthM|Clear Width (m)|Lichte Breite (m)
clearance|Clearance|Freihaltefläche|Free space needed around the equipment for use and maintenance.|Freizuhaltende Fläche um die Ausstattung für Bedienung und Wartung.
client|Client Application|Client-Anwendung
clientName|Client|Auftraggeber
climateAdaptation|Climate Adaptation|Klimaanpassung
climateZone|Climate Zone|Klimazone
closeDate|Close Date|Abschlussdatum
code|Code|Kennung
ProgramElement.code|Room Code|Raumnummer
ProjectDefinition.code|Project Code|Projektkennung
cognitiveLoad|Cognitive Load|Kognitive Belastung
cognitiveProfile|Cognitive Profile|Kognitives Profil
collaborationModel|Collaboration Model|Kooperationsmodell
collectionYear|Collection Year|Erhebungsjahr
colorCoding|Colour Coding|Farbcodierung
comfortBand|Comfort Band|Behaglichkeitsbereich|Acceptable comfort range, e.g. a category per DIN EN 16798-1.|Zulässiger Behaglichkeitsbereich, z. B. eine Kategorie nach DIN EN 16798-1.
comments|Comments|Kommentare
commissioning|Commissioning|Inbetriebnahme
commissioningNotes|Commissioning Notes|Hinweise zur Inbetriebnahme
commissioningWindow|Commissioning Window|Inbetriebnahmezeitraum
communicationChannels|Communication Channels|Kommunikationskanäle
communicationPlan|Communication Plan|Kommunikationsplan
communicationPreferences|Communication Preferences|Kommunikationspräferenzen
companionSeating|Companion Seating|Begleitplätze
comparisonNotes|Comparison Notes|Vergleichshinweise
compatibilityRequirement|Compatibility Requirement|Verträglichkeitsanforderung
completionCriteria|Completion Criteria|Abschlusskriterien
complianceMethod|Compliance Method|Nachweisverfahren
complianceObligations|Compliance Obligations|Compliance-Pflichten
complianceStatus|Compliance Status|Konformitätsstatus
complianceTags|Compliance Tags|Compliance-Schlagwörter
concerns|Concerns|Anliegen
conditions|Conditions|Bedingungen
ApprovalRecord.conditions|Conditions|Auflagen
confidence|Confidence|Konfidenz
confidenceLevel|Confidence Level|Vertrauensniveau
confidentiality|Confidentiality|Vertraulichkeit
conflictIds|Conflicts|Konflikte
conflicts|Conflicts|Konflikte
connection|Connection|Verbindung
consentProcess|Consent Process|Einwilligungsverfahren
constraintDetails|Constraint Details|Details der Randbedingung
constraintStatus|Constraint Status|Status der Randbedingung
constraintType|Constraint Type|Art der Randbedingung
constraints|Constraints|Randbedingungen
constraintsSummary|Constraints Summary|Übersicht der Randbedingungen
consultantIds|Consulted|Konsultiert|Stakeholders consulted on this entry.|Beteiligte, die zu diesem Eintrag konsultiert werden.
consultantRefs|Consultants|Berater
consultedIds|Consulted|Konsultiert
contactEmail|Email|E-Mail
contactPhone|Phone|Telefon
content|Content|Inhalt
contentRef|Content Reference|Inhaltsverweis
context|Context|Ausgangslage
contingency|Contingency Measures|Notfallmaßnahmen
contingencyDays|Contingency (days)|Zeitreserve (Tage)
contingencyPercent|Contingency (%)|Kostenreserve (%)|Reserve for unforeseen costs as a share of the amount.|Reserve für Unvorhergesehenes als Anteil am Betrag.
contingencyPlan|Contingency Plan|Notfallplan
continuousImprovement|Continuous Improvement|Kontinuierliche Verbesserung
contractRefs|Contracts|Verträge
contractionScenario|Contraction Scenarios|Schrumpfungsszenarien
controlKind|Control Kind|Art der Sicherungsmaßnahme
controlsHeight|Controls Height|Bedienhöhe|Mounting height of controls, e.g. 85 cm per DIN 18040.|Montagehöhe von Bedienelementen, z. B. 85 cm nach DIN 18040.
correctiveActionProcess|Corrective Action Process|Korrekturmaßnahmenprozess
correctiveActions|Corrective Actions|Korrekturmaßnahmen
correlationId|Correlation ID|Korrelations-ID
costDelta|Cost Change|Kostenänderung
costEstimate|Cost Estimate|Kostenschätzung
costImpact|Cost Impact|Kostenauswirkung
costItem|Cost Item|Kostenposition|Cost group or item, e.g. a cost group per DIN 276.|Kostengruppe oder Position, z. B. eine Kostengruppe nach DIN 276.
costModel|Cost Model|Kostenmodell
costOfChange|Cost of Change|Umbaukosten
costPerUnit|Cost per Unit|Kosten je Einheit
created|Created|Erstellt am
createdBy|Created By|Erstellt von
criteria|Criteria|Kriterien
criteriaIds|Criteria|Kriterien
criterion|Criterion|Kriterium
critical|Critical|Kritisch
criticalPath|Critical Path|Kritischer Pfad
criticality|Criticality|Kritikalität
culturalConsiderations|Cultural Considerations|Kulturelle Aspekte
cultureNotes|Culture Notes|Hinweise zur Unternehmenskultur
currency|Currency|Währung
current|Current|Istwert
customerProfiles|Customer Profiles|Kundenprofile
customization|Customisation|Anpassung
customizationNotes|Customisation Notes|Anpassungshinweise
cybersecurity|Cyber Security|IT-Sicherheit
dataGovernance|Data Governance|Datenverwaltung
dataPrivacy|Data Privacy|Datenschutz
dataSource|Data Source|Datenquelle
dateFrom|Date From|Datum von
dateTo|Date To|Datum bis
daylightRequirement|Daylight Requirement|Tageslichtanforderung|e.g. a daylight provision level per DIN EN 17037.|z. B. eine Tageslichtversorgungsstufe nach DIN EN 17037.
daylightTarget|Daylight Target|Tageslichtziel
decantRequirements|Decant Requirements|Anforderungen an die Interimsauslagerung
decisionAuthority|Decision Authority|Entscheidungsbefugnis
decisionCriteria|Decision Criteria|Entscheidungskriterien
decisionDate|Decision Date|Entscheidungsdatum
decisionId|Decision|Entscheidung
decisionIds|Decisions|Entscheidungen
decisionMakerIds|Decision Makers|Entscheidungsträger
decisionMaking|Decision Making|Entscheidungsfindung
decisionPoints|Decision Points|Entscheidungspunkte
WayfindingRequirement.decisionPoints|Decision Points|Entscheidungspunkte|Places where users choose their direction.|Orte, an denen Nutzende ihre Richtung wählen.
decisionRights|Decision Rights|Entscheidungsrechte
decisionStatement|Decision Statement|Beschluss
decisions|Decisions|Entscheidungen
decisionsMade|Decisions Made|Getroffene Entscheidungen
decommissionPlan|Decommissioning Plan|Stilllegungsplan
defaultFields|Default Fields|Standardfelder
defectCategories|Defect Categories|Mängelkategorien
delegatedTo|Delegated To|Delegiert an
delegationChain|Delegation Chain|Delegationskette
deliverables|Deliverables|Liefergegenstände
demographic|Demographic|Demografie
demountablePartitions|Demountable Partitions|Demontierbare Trennwände
department|Department|Abteilung
dependencies|Dependencies|Abhängigkeiten
description|Description|Beschreibung
destinationSystems|Destination Systems|Zielsysteme
destinationTypes|Destination Types|Zielarten
details|Details|Details
detectedBy|Detected By|Erkannt von
detectionDate|Detection Date|Erkennungsdatum
developmentContext|Development Context|Entwicklungskontext
digitalWayfinding|Digital Wayfinding|Digitales Leitsystem
dimensions|Dimensions|Abmessungen
direction|Direction|Richtung
directional|Directed|Gerichtet
disabilities|Impairments|Beeinträchtigungen
disasterRecovery|Disaster Recovery|Notfallwiederherstellung
disposalNotes|Disposal Notes|Entsorgungshinweise
distanceConstraintM|Distance Limit (m)|Abstandsgrenze (m)
distanceMaxM|Maximum Distance (m)|Maximaler Abstand (m)
distanceMinM|Minimum Distance (m)|Minimaler Abstand (m)
distribution|Distribution|Verteilung
distributionChannels|Distribution Channels|Verteilkanäle
distributionList|Distribution List|Verteiler
diversityFactor|Diversity Factor|Gleichzeitigkeitsfaktor|Share of the connected load expected at the same time.|Anteil der Anschlussleistung, der gleichzeitig auftritt.
diversityGoals|Diversity Goals|Diversitätsziele
documentControl|Document Control|Dokumentenlenkung
documentId|Document ID|Dokument-ID
documentIds|Documents|Dokumente
documentStatus|Document Status|Dokumentstatus
documentType|Document Type|Dokumentart
documentationRequirements|Documentation Requirements|Dokumentationsanforderungen
drawbacks|Drawbacks|Nachteile
drillFrequency|Drill Frequency|Übungshäufigkeit
drillRequirements|Drill Requirements|Übungsanforderungen
dueDate|Due Date|Fälligkeitsdatum
durability|Durability|Haltbarkeit
duration|Duration|Dauer
durationMs|Duration (ms)|Dauer (ms)
effectiveDate|Effective Date|Gültig ab
effectiveFrom|Effective From|Gültig ab
effectiveUntil|Effective Until|Gültig bis
effects|Effects|Auswirkungen
electricalSafety|Electrical Safety|Elektrische Sicherheit
elementAId|First Element|Erstes Element
elementBId|Second Element|Zweites Element
elementIds|Program Elements|Programmelemente
elevationM|Elevation (m)|Geländehöhe (m)|Site elevation above the height datum.|Geländehöhe über dem Höhenbezugssystem.
embodiedCarbon|Embodied Carbon|Graue Emissionen|Embodied greenhouse gas emissions (CO₂e) of construction.|Graue Treibhausgasemissionen (CO₂-Äq.) der Errichtung.
emergencyEgress|Emergency Egress|Rettungswege
emergencyEvacuation|Emergency Evacuation|Evakuierung im Notfall
emergencyOverride|Emergency Override|Notfallfreigabe
emergencyProcedures|Emergency Procedures|Notfallverfahren
emergencyRoute|Emergency Route|Rettungsweg
emergencyUse|Emergency Use|Nutzung im Notfall
enclosureRequired|Enclosure Required|Raumabschluss erforderlich
endDate|End Date|Enddatum
endTime|End Time|Endzeit
energyImplications|Energy Implications|Energetische Auswirkungen
energyStrategy|Energy Strategy|Energiekonzept
enforcementMethod|Enforcement Method|Durchsetzungsmethode
engagement|Engagement|Engagement
entityAId|First Entity|Erste Entität
entityBId|Second Entity|Zweite Entität
entityKinds|Entity Kinds|Entitätsarten
entryPoints|Service Entry Points|Hausanschlusspunkte
environmentalConstraints|Environmental Constraints|Umweltrestriktionen
environmentalNeeds|Environmental Needs|Anforderungen an die Umgebung
environmentalZone|Environmental Zone|Raumklimazone
equipmentClearance|Equipment Clearance|Durchgangsmaß für Geräte
equipmentIds|Equipment|Ausstattung
ergonomicCriteria|Ergonomic Criteria|Ergonomische Kriterien
ergonomicNotes|Ergonomic Notes|Ergonomische Hinweise
ergonomicsRating|Ergonomics Rating|Ergonomiebewertung
errorMessage|Error Message|Fehlermeldung
escalationContactId|Escalation Contact|Eskalationskontakt
escalationLevel|Escalation Level|Eskalationsstufe
escalationPath|Escalation Path|Eskalationsweg
escalationPaths|Escalation Paths|Eskalationswege
escalationRate|Escalation Rate|Preissteigerungsrate|Expected annual construction cost escalation.|Erwartete jährliche Baupreissteigerung.
escortPolicy|Escort Policy|Begleitregelung
escortRequired|Escort Required|Begleitung erforderlich
ethicsPolicy|Ethics Policy|Ethikrichtlinien
evacuationRequirements|Evacuation Requirements|Räumungsanforderungen
evaluationDate|Evaluation Date|Bewertungsdatum
evaluationStatus|Evaluation Status|Bewertungsstatus
evaluatorIds|Evaluators|Bewertende
evidence|Evidence|Nachweise
evidenceRefs|Evidence|Nachweise
evidenceRequired|Required Evidence|Erforderliche Nachweise
exceptionManagement|Exception Management|Ausnahmemanagement
exceptions|Exceptions|Ausnahmen
exemptions|Exemptions|Befreiungen
expansionDirection|Expansion Directions|Erweiterungsrichtungen
expansionElementIds|Expansion Elements|Erweiterungselemente
expectations|Expectations|Erwartungen
expertiseLevel|Expertise Level|Fachniveau
expirationDate|Expiration Date|Ablaufdatum
expiryDate|Expiry Date|Ablaufdatum
exportProfile|Export Profile|Exportprofil
facilitatorId|Facilitator|Moderation
factor|Factor|Faktor
failureImpact|Failure Impact|Ausfallfolgen
failureModes|Failure Modes|Fehlerarten
feedback|Feedback|Rückmeldungen
feedbackChannels|Feedback Channels|Rückmeldekanäle
feedbackLoop|Feedback Loop|Rückkopplung
fileRef|File Reference|Dateiverweis
filterDescription|Filter Description|Filterbeschreibung
filterName|Filter Name|Filtername
findings|Findings|Befunde
fireProtection|Fire Protection|Brandschutz
flexibilityNotes|Flexibility Notes|Hinweise zur Flexibilität
flexibilityType|Flexibility Type|Flexibilitätsart
floatDays|Float (days)|Pufferzeit (Tage)|Total float before the finish date moves.|Gesamtpuffer, bevor sich der Endtermin verschiebt.
floodRisk|Flood Risk|Hochwasserrisiko
flowType|Flow Type|Flussart
followUpActions|Follow-Up Actions|Folgemaßnahmen
followUpDate|Follow-Up Date|Nachverfolgungsdatum
forecast|Forecast|Prognose
format|Format|Format|Text format hint, e.g. plain or markdown.|Hinweis auf das Textformat, z. B. plain oder markdown.
ArtifactRecord.format|File Format|Dateiformat
InformationRequirement.format|Data Format|Datenformat
ReportRecord.format|Output Format|Ausgabeformat
framework|Governance Framework|Governance-Rahmenwerk|e.g. ISO 9001 or ISO 41001.|z. B. ISO 9001 oder ISO 41001.
frequency|Frequency|Häufigkeit
fromElementId|From Element|Von Element
fromId|From|Von
functionIds|Functions|Funktionen
fundingModel|Funding Model|Finanzierungsmodell
fundingSource|Funding Source|Finanzierungsquelle
fundingSources|Funding Sources|Finanzierungsquellen
furnitureClass|Furniture Class|Möbelklasse
furnitureStrategy|Furniture Strategy|Möblierungskonzept
futureChanges|Future Changes|Künftige Änderungen
futureExpansion|Future Expansion|Künftige Erweiterung
futureFunctionIds|Future Functions|Künftige Funktionen
gapAnalysis|Gap Analysis|Lückenanalyse
generatedAt|Generated At|Erzeugt am
generatedBy|Generated By|Erzeugt von
geographicContext|Geographic Context|Geografischer Kontext
geography|Region|Region
goals|Goals|Ziele
governancePerformance|Governance Performance|Governance-Kennzahlen
growthAllocation|Growth Allocation|Wachstumsreserve
growthAllowance|Growth Allowance|Wachstumszuschlag
growthFactor|Growth Factor|Wachstumsfaktor
growthPlanId|Growth Plan|Wachstumsplan
growthRate|Growth Rate|Wachstumsrate
handlingEquipment|Handling Equipment|Fördermittel
handoffPoints|Handoff Points|Übergabepunkte
hardDeadline|Hard Deadline|Verbindlicher Termin
hardeningMeasures|Hardening Measures|Härtungsmaßnahmen
hazard|Hazard|Gefährdung
hazardClass|Hazard Class|Gefahrenklasse|e.g. a storage class per TRGS 510.|z. B. eine Lagerklasse nach TRGS 510.
headcount|Headcount|Personalstärke
headcountDelta|Headcount Change|Änderung der Personalstärke
headcountGrowth|Headcount Growth|Personalwachstum
health|Health|Zustand
hearingLoop|Hearing Loop|Induktive Höranlage
height|Room Height|Raumhöhe
heritageConstraints|Heritage Constraints|Denkmalschutzauflagen
hierarchyLevels|Hierarchy Levels|Hierarchieebenen
hierarchyParentId|Parent Function|Übergeordnete Funktion
horizonYears|Horizon (years)|Planungshorizont (Jahre)
humidityRange|Humidity Range|Luftfeuchtebereich
hypothesis|Hypothesis|Hypothese
iaqTarget|Indoor Air Quality Target|Zielwert der Innenraumluftqualität
id|ID|ID
impact|Impact|Schadensausmaß
impactAssessment|Impact Assessment|Auswirkungsanalyse
impactIfFalse|Impact If False|Auswirkung bei Nichtzutreffen
impactSummary|Impact Summary|Zusammenfassung der Auswirkungen
impactedElementIds|Impacted Elements|Betroffene Elemente
impactedEntityIds|Impacted Entities|Betroffene Entitäten
impactedRequirementIds|Impacted Requirements|Betroffene Anforderungen
improvementOpportunities|Improvement Opportunities|Verbesserungspotenziale
incentiveThreshold|Incentive Threshold|Bonusschwelle
incidentReporting|Incident Reporting|Meldung von Vorfällen
incompatibilityRequirement|Incompatibility Requirement|Unverträglichkeitsanforderung
industrySector|Industry Sector|Branche
influence|Influence|Einfluss
influenceStrategy|Influence Strategy|Einflussstrategie
informationType|Information Type|Informationsart
informedIds|Informed|Informiert
infrastructureHeadroom|Infrastructure Headroom|Infrastrukturreserven
infrastructureIds|Infrastructure|Infrastruktur
infrastructureSpareCapacity|Infrastructure Spare Capacity|Reservekapazität der Infrastruktur
inputEntityIds|Input Entities|Eingangsentitäten
inputs|Inputs|Eingaben
inspectionFrequency|Inspection Frequency|Prüfintervall
inspectionPoints|Inspection Points|Prüfpunkte
installationRequirements|Installation Requirements|Einbauanforderungen
insuranceImplications|Insurance Implications|Versicherungsaspekte
integrationPoints|Integration Points|Schnittstellen
intensity|Intensity|Intensität
interest|Interest|Interesse
interfaceRequirements|Interface Requirements|Schnittstellenanforderungen
interfaces|Interfaces|Schnittstellen
internalExternalAccess|Internal or External Access|Innen- oder Außenzugang
interpretationNotes|Interpretation Notes|Auslegungshinweise
intrusionDetection|Intrusion Detection|Einbruchmeldetechnik
involvementPhases|Involvement Phases|Beteiligungsphasen
ipAddress|IP Address|IP-Adresse
isPublic|Public|Öffentlich
issueDate|Issue Date|Ausgabedatum
issueDescription|Issue Description|Beschreibung des offenen Punkts
issueIds|Issues|Offene Punkte
issuePriority|Issue Priority|Priorität des offenen Punkts
issueType|Issue Type|Art des offenen Punkts
issues|Issues|Offene Punkte
jurisdiction|Jurisdiction|Zuständigkeitsbereich
kind|Kind|Art
keyManagement|Key Management|Schlüsselverwaltung
keywords|Keywords|Schlüsselwörter
knowledgeId|Knowledge Record|Wissenseintrag
kpiTargets|KPI Targets|Kennzahlenziele
kpis|KPIs|Kennzahlen
label|Label|Beschriftung
landmarkStrategy|Landmark Strategy|Landmarkenkonzept
language|Languages|Sprachen
languages|Languages|Sprachen
lastApplied|Last Applied|Zuletzt angewendet
lastReviewed|Last Reviewed|Zuletzt geprüft
lastUsed|Last Used|Zuletzt verwendet
lastVerified|Last Verified|Zuletzt verifiziert
latitude|Latitude|Breitengrad
launchDate|Launch Date|Startdatum
leaseImplications|Lease Implications|Mietvertragliche Auswirkungen
lessonsLearned|Lessons Learned|Erkenntnisse
AccessibilityRequirement.level|Accessibility Level|Barrierefreiheitsstufe
PrivacyRequirement.level|Privacy Level|Privatsphärestufe
ProgramElement.level|Storey|Geschoss
levelConstraint|Storey Constraint|Geschossvorgabe
liaisonContacts|Liaison Contacts|Ansprechstellen
license|Licence|Lizenz
lifecycleCost|Life-Cycle Cost|Lebenszykluskosten
lifecycleYears|Service Life (years)|Nutzungsdauer (Jahre)
liftRequired|Lift Required|Aufzug erforderlich
lightingForTasks|Task Lighting|Arbeitsplatzbeleuchtung
lightingRequirements|Lighting Requirements|Beleuchtungsanforderungen
limitations|Limitations|Einschränkungen
linkedRequirementIds|Linked Requirements|Verknüpfte Anforderungen
linkedRiskIds|Linked Risks|Verknüpfte Risiken
locale|Locale|Gebietsschema
location|Location|Ort
locationContext|Location Context|Ortsbezug
locationHint|Location Hint|Lagehinweis
longitude|Longitude|Längengrad
machinerySafety|Machinery Safety|Maschinensicherheit
maintenanceAccess|Maintenance Access|Wartungszugang
maintenanceInterval|Maintenance Interval|Wartungsintervall
maintenancePlan|Maintenance Plan|Wartungsplan
manufacturer|Manufacturer|Hersteller
materials|Materials|Materialien
materialsPreferences|Material Preferences|Materialpräferenzen
max|Maximum|Maximum
maxCoverage|Maximum Site Coverage|Maximaler Überbauungsgrad|Largest share of the site that may be built over, e.g. the GRZ per § 19 BauNVO.|Höchstzulässiger überbaubarer Anteil des Grundstücks, z. B. die GRZ nach § 19 BauNVO.
maxHeightM|Maximum Building Height (m)|Maximale Gebäudehöhe (m)
maxValue|Maximum Value|Höchstwert
maximum|Maximum|Höchstwert
maximumSignageDistanceM|Maximum Signage Distance (m)|Maximaler Schilderabstand (m)
measurementMethod|Measurement Method|Messverfahren
medium|Media|Medien
meetingCadence|Meeting Cadence|Sitzungsrhythmus
meetingRef|Meeting|Besprechung
meetingStatus|Meeting Status|Besprechungsstatus
meetingType|Meeting Type|Besprechungsart
messageTypes|Message Types|Nachrichtenarten
metadataRequirements|Metadata Requirements|Metadatenanforderungen
method|Method|Methode
methodology|Methodology|Methodik
methods|Methods|Methoden
metric|Metric|Kennzahl
metrics|Metrics|Kennzahlen
milestone|Milestone|Meilenstein
milestoneId|Milestone|Meilenstein
min|Minimum|Minimum
minValue|Minimum Value|Mindestwert
minimum|Minimum|Mindestwert
minutes|Minutes|Protokoll
mission|Mission|Mission
mitigation|Mitigation|Risikominderung
mitigationMeasures|Mitigation Measures|Schutzmaßnahmen
mitigationOptions|Mitigation Options|Minderungsoptionen
mobility|Mobility|Mobilität
mobilityProfile|Mobility Profile|Mobilitätsprofil
model|Model|Modell
modularityLevel|Modularity Level|Modularitätsgrad
monitoring|Monitoring|Überwachung
monitoringFrequency|Monitoring Frequency|Überwachungsintervall
monitoringMethod|Monitoring Method|Überwachungsmethode
monitoringPlan|Monitoring Plan|Überwachungsplan
monitoringRequired|Monitoring Required|Überwachung erforderlich
monitoringRestrictions|Monitoring Restrictions|Überwachungsbeschränkungen
mounting|Mounting|Montageart
multiUsePotential|Multi-Use Potential|Mehrfachnutzungspotenzial
name|Name|Bezeichnung
neighbors|Neighbouring Buildings|Nachbarbebauung
nextActions|Next Actions|Nächste Schritte
nextReview|Next Review|Nächste Prüfung
nextReviewDate|Next Review Date|Nächster Prüftermin
noiseLevelDb|Noise Level (dB)|Schallpegel (dB)
noiseRestrictions|Noise Restrictions|Lärmschutzauflagen
noiseSources|Noise Sources|Lärmquellen
nonConformities|Non-Conformities|Abweichungen
normalized|Normalised|Normalisiert|Whether the element pair is stored in canonical order.|Ob das Elementpaar in kanonischer Reihenfolge gespeichert ist.
notes|Notes|Notizen
notificationList|Notification List|Benachrichtigungsliste
objectives|Objectives|Ziele
obligation|Obligation|Verpflichtung
observationRisk|Observation Risk|Einsehbarkeitsrisiko
occupancy|Occupancy|Belegung|Number of occupants the element accommodates.|Anzahl der Personen, für die das Element ausgelegt ist.
occupancyBasis|Occupancy Basis|Belegungsgrundlage
occupancyConstraints|Occupancy Constraints|Einschränkungen im laufenden Betrieb
occupancyImpact|Occupancy Impact|Auswirkungen auf den Betrieb
occupation|Occupation|Beruf
operatingHours|Operating Hours|Betriebszeiten
operation|Operation|Betriebsablauf
operationalCarbon|Operational Carbon|Betriebsbedingte Emissionen|Greenhouse gas emissions (CO₂e) of operation.|Treibhausgasemissionen (CO₂-Äq.) des Betriebs.
operationalContext|Operational Context|Betrieblicher Kontext
optionDescription|Option Description|Variantenbeschreibung
optionIds|Options|Varianten
optionName|Option Name|Variantenbezeichnung
optionsConsidered|Options Considered|Betrachtete Varianten
organization|Organisation|Organisation
organizationSystem|Organisation System|Ordnungssystem
orientation|Orientation|Ausrichtung
outcomes|Outcomes|Ergebnisse
outdoorConditions|Outdoor Conditions|Außenklimabedingungen
outputSummary|Output Summary|Ergebniszusammenfassung
outputs|Outputs|Ergebnisse
overheadServices|Overhead Services|Versorgung über die Decke
ownerId|Owner|Verantwortlich|The stakeholder responsible for this entry.|Der Beteiligte, der für diesen Eintrag verantwortlich ist.
ownerIds|Owners|Verantwortliche
ownerOrganization|Owner Organisation|Eigentümerorganisation
ownerStakeholderId|Owner|Verantwortlich
ownership|Ownership|Zuständigkeiten
painPoints|Pain Points|Problempunkte
parameter|Parameter|Parameter
parameterKind|Parameter Kind|Parameterart
parameters|Parameters|Parameter
parentId|Parent Element|Übergeordnetes Element
parentRequirementId|Parent Requirement|Übergeordnete Anforderung
participantIds|Participants|Mitwirkende|Stakeholders taking part in this entry.|Beteiligte, die an diesem Eintrag mitwirken.
participants|Participants|Teilnehmende
Activity.participants|Participants|Teilnehmerzahl
peak|Peak|Spitzenwert
peakDemand|Peak Demand|Spitzenbedarf
peakFactor|Peak Factor|Spitzenfaktor
peakPeriods|Peak Periods|Spitzenzeiten
peakRate|Peak Rate|Spitzenrate
peakUsageTimes|Peak Usage Times|Hauptnutzungszeiten
penalties|Penalties|Sanktionen
penaltyClauses|Penalty Clauses|Vertragsstrafen
penaltyThreshold|Penalty Threshold|Malusschwelle
performanceIndicators|Performance Indicators|Leistungskennzahlen
performanceTargets|Performance Targets|Leistungsziele
perimeterControls|Perimeter Controls|Perimetersicherung
personaArchetype|Persona Archetype|Persona-Archetyp
phase|Project Phase|Projektphase
phases|Phases|Phasen
phasingStrategy|Phasing Strategy|Bauabschnittskonzept
pinned|Pinned|Angeheftet
policyOwnershipId|Policy Owner|Verantwortlich für Richtlinien
postureRequirements|Posture Requirements|Anforderungen an die Körperhaltung
powerInterestNotes|Power–Interest Notes|Hinweise zur Einfluss-Interessen-Matrix
powerKw|Power (kW)|Leistung (kW)
ppeRequirements|PPE Requirements|Anforderungen an die persönliche Schutzausrüstung
predecessors|Predecessors|Vorgänger
preferences|Preferences|Präferenzen
preferred|Preferred|Bevorzugt
previousStatus|Previous Status|Vorheriger Status
priorities|Priorities|Prioritäten
priority|Priority|Priorität
privacyControls|Privacy Controls|Datenschutzmaßnahmen
privacyKind|Privacy Kind|Öffentlichkeitsgrad
privacyNeeds|Privacy Needs|Anforderungen an die Privatsphäre
privacyType|Privacy Type|Privatsphäretyp
probability|Probability|Wahrscheinlichkeit
Risk.probability|Likelihood|Eintrittswahrscheinlichkeit
problemStatement|Problem Statement|Problemstellung
processId|Process|Prozess
processIds|Processes|Prozesse
procurementLeadTime|Procurement Lead Time|Beschaffungsvorlaufzeit
progressPercent|Progress (%)|Fortschritt (%)
projectPriorities|Project Priorities|Projektprioritäten
projectType|Project Type|Projektart
provider|Provider|Anbieter
proximityRequirement|Proximity Requirement|Näheanforderung
publicTransit|Public Transport|ÖPNV-Anbindung
purpose|Purpose|Zweck
qualityCriteria|Quality Criteria|Qualitätskriterien
qualityGates|Quality Gates|Qualitätstore
qualityImpact|Quality Impact|Qualitätsauswirkungen
qualityMetrics|Quality Metrics|Qualitätskennzahlen
qualityPolicy|Quality Policy|Qualitätspolitik
qualityTopic|Quality Topic|Qualitätsthema
quantity|Quantity|Menge
quantityBasis|Quantity Basis|Mengenbasis
quantityIds|Quantity Requirements|Mengenanforderungen
questions|Questions|Fragen
queueManagement|Queue Management|Warteschlangenmanagement
quorumMet|Quorum Met|Beschlussfähig
raisedFloor|Raised Floor|Doppelboden
rampSlope|Ramp Slope|Rampenneigung|Maximum ramp gradient; DIN 18040 limits it to 6 %.|Maximale Rampenneigung; DIN 18040 begrenzt sie auf 6 %.
rank|Rank|Rang
rankedPriority|Ranked Priority|Eingestufte Priorität
rankingNotes|Ranking Notes|Hinweise zur Rangfolge
rationale|Rationale|Begründung
rawResultRef|Raw Result Reference|Verweis auf das Rohergebnis
reachEnvelope|Reach Envelope|Greifraum
reason|Reason|Grund
recommendation|Recommendation|Empfehlung
recommendations|Recommendations|Empfehlungen
recommendedResolution|Recommended Resolution|Empfohlene Lösung
reconfigurationTime|Reconfiguration Time|Umrüstzeit
recordStatus|Record Status|Eintragsstatus
recordingPolicy|Recording Policy|Aufzeichnungsrichtlinie
recordingRef|Recording Reference|Verweis auf die Aufzeichnung
recoveryPoint|Recovery Point Objective|Wiederherstellungspunkt (RPO)
recoveryTime|Recovery Time Objective|Wiederanlaufzeit (RTO)
redundancy|Redundancy|Redundanz
references|References|Referenzen
regulatoryBasis|Regulatory Basis|Rechtsgrundlage
regulatoryBody|Regulatory Body|Aufsichtsbehörde
regulatoryConsiderations|Regulatory Considerations|Regulatorische Aspekte
regulatoryContext|Regulatory Context|Regulatorischer Rahmen
regulatoryRefs|Regulatory References|Regelwerksverweise
rejectionReason|Rejection Reason|Ablehnungsgrund
relatedChangeId|Related Change|Zugehörige Änderung
relatedConflictIds|Related Conflicts|Zugehörige Konflikte
relatedDecisionId|Related Decision|Zugehörige Entscheidung
relatedDecisionIds|Related Decisions|Zugehörige Entscheidungen
relatedEntityIds|Related Entities|Zugehörige Entitäten
relatedEntityKinds|Related Entity Kinds|Zugehörige Entitätsarten
relatedIssueIds|Related Issues|Zugehörige offene Punkte
relatedKnowledgeIds|Related Knowledge|Zugehöriges Wissen
relatedRequirementIds|Related Requirements|Zugehörige Anforderungen
relatedRiskIds|Related Risks|Zugehörige Risiken
relationshipPriority|Relationship Priority|Priorität der Beziehung
relationshipToClient|Relationship to Client|Beziehung zum Auftraggeber
relocationStrategy|Relocation Strategy|Umzugskonzept
remediationPlan|Remediation Plan|Behebungsplan
replacementCost|Replacement Cost|Wiederbeschaffungskosten
replacementCycle|Replacement Cycle|Erneuerungszyklus
reportId|Report|Bericht
reporterId|Reporter|Meldende Person
reportingCadence|Reporting Cadence|Berichtsrhythmus
reportingFrequency|Reporting Frequency|Berichtshäufigkeit
reportingLine|Reporting Line|Berichtslinie
reportingPeriod|Reporting Period|Berichtszeitraum
reportingRequirements|Reporting Requirements|Berichtspflichten
representativeOf|Representative Of|Vertritt
requestedBy|Requested By|Beantragt von
requirementIds|Requirements|Anforderungen
requirementOwnershipId|Requirements Owner|Verantwortlich für Anforderungen
requirementText|Requirement Text|Anforderungstext
researchBasis|Research Basis|Forschungsgrundlage
researchMethod|Research Method|Erhebungsmethode
reserveAreas|Reserve Areas|Reserveflächen
residualImpact|Residual Impact|Restschadensausmaß
residualProbability|Residual Likelihood|Resteintrittswahrscheinlichkeit
residualRisk|Residual Risk|Restrisiko
resolution|Resolution|Lösung
resolutionPlan|Resolution Plan|Lösungsplan
resolutionStatus|Resolution Status|Lösungsstatus
resolvedDate|Resolved Date|Erledigt am
resourceIds|Resources|Ressourcen
resourceRequirements|Resource Requirements|Ressourcenbedarf
resourceType|Resource Type|Ressourcenart
responseCount|Responses|Anzahl der Antworten
responseProcedures|Response Procedures|Reaktionsverfahren
responseRate|Response Rate|Rücklaufquote
responseTime|Response Time|Reaktionszeit
responsibilities|Responsibilities|Verantwortlichkeiten
responsibleParty|Responsible Party|Verantwortliche Stelle
result|Result|Ergebnis
resubmissionDate|Resubmission Date|Wiedervorlagedatum
retentionPeriod|Retention Period|Aufbewahrungsfrist
retentionPolicy|Retention Policy|Aufbewahrungsrichtlinie
retentionUntil|Retain Until|Aufbewahren bis
reversalConditions|Reversal Conditions|Bedingungen für eine Revision
reviewCycle|Review Cycle|Überprüfungszyklus
reviewDate|Review Date|Überprüfungsdatum
reviewHierarchy|Review Hierarchy|Prüfhierarchie
reviewerIds|Reviewers|Prüfende
revision|Revision|Revision
revisionDate|Revision Date|Revisionsdatum
revocationPolicy|Revocation Policy|Entzugsregelung
riskAppetite|Risk Appetite|Risikobereitschaft
riskFactors|Risk Factors|Risikofaktoren
riskIds|Risks|Risiken
riskImpact|Risk Impact|Risikoauswirkungen
riskLevel|Risk Level|Risikostufe
riskOwnershipId|Risk Owner|Risikoverantwortlich
riskScore|Risk Score|Risikokennzahl
riskStatement|Risk Statement|Risikobeschreibung
riskSummary|Risk Summary|Risikoübersicht
role|Role|Rolle
roleTitle|Job Title|Funktionsbezeichnung
roles|Roles|Rollen
rollbackPlan|Rollback Plan|Rückfallplan
rootCause|Root Cause|Grundursache
routeIds|Routes|Wege
runAt|Run At|Ausgeführt am
runBy|Run By|Ausgeführt von
safetyDomain|Safety Domain|Sicherheitsbereich
sampleRate|Sample Rate|Stichprobenrate
sampleSize|Sample Size|Stichprobengröße
scenario|Scenario|Szenario
scenarioId|Scenario|Szenario
scenarioIds|Scenarios|Szenarien
scheduleConstraint|Schedule Constraint|Terminvorgabe
scheduleDelta|Schedule Change|Terminänderung
scheduleEstimate|Schedule Estimate|Terminschätzung
scheduleImpact|Schedule Impact|Terminauswirkung
schedulePhase|Schedule Phase|Terminphase
scheduledDate|Scheduled Date|Geplantes Datum
scheduledEnd|Scheduled End|Geplantes Ende
scheduledStart|Scheduled Start|Geplanter Beginn
schema|Schema|Schema
scopeExclusions|Scope Exclusions|Nicht enthaltene Leistungen
scopeInclusions|Scope Inclusions|Enthaltene Leistungen
score|Score|Punktzahl
scores|Scores|Punktzahlen
scoringMethod|Scoring Method|Bewertungsmethode
screening|Screening|Sicherheitskontrollen
screeningRequired|Visual Screening Required|Sichtschutz erforderlich
seasonalVariation|Seasonal Variation|Jahreszeitliche Schwankung
sections|Sections|Abschnitte
sector|Sector|Branche
securityLevel|Security Level|Sicherheitsstufe
securityZone|Security Zone|Sicherheitszone
seismicZone|Seismic Zone|Erdbebenzone|e.g. an earthquake zone per DIN EN 1998-1/NA.|z. B. eine Erdbebenzone nach DIN EN 1998-1/NA.
selectedOptionId|Selected Option|Gewählte Variante
sensitivityFactors|Sensitivity Factors|Sensitivitätsfaktoren
sensoryProfile|Sensory Profile|Sensorisches Profil
separationRequirements|Separation Requirements|Trennungsanforderungen
separations|Separations|Trennungen
sequencing|Sequencing|Abfolge
serviceAnimalPolicy|Assistance Dog Policy|Regelung für Assistenzhunde
serviceCategory|Service Category|Leistungskategorie
serviceLevel|Service Level|Servicelevel
serviceName|Service Name|Leistungsbezeichnung
serviceRequirementIds|Service Requirements|Dienstleistungsanforderungen
serviceType|Service Type|Leistungsart
sessionId|Session ID|Sitzungs-ID
sessionType|Session Type|Sitzungsart
severity|Severity|Schweregrad
sharedEntry|Shared Entry|Gemeinsamer Zugang
sharedWall|Shared Wall|Gemeinsame Wand
sharingModel|Sharing Model|Teilungsmodell
sharingRatio|Sharing Ratio|Teilungsverhältnis|Users per unit when the resource is shared, e.g. a desk-sharing ratio.|Nutzende je Einheit bei geteilter Nutzung, z. B. eine Desksharing-Quote.
shiftPattern|Shift Pattern|Schichtmodell
signage|Signage|Beschilderung
signageLocations|Signage Locations|Schilderstandorte
signageRequired|Signage Required|Beschilderung erforderlich
signageRequirements|Signage Requirements|Anforderungen an die Beschilderung
signageTypes|Signage Types|Schilderarten
signatureMethod|Signature Method|Signaturverfahren
siteLogistics|Site Logistics|Baustellenlogistik
siteName|Site Name|Standortbezeichnung
slaTarget|SLA Target|SLA-Zielwert
slipTripFall|Slips, Trips and Falls|Ausrutschen, Stolpern und Stürzen
socialInteraction|Social Interaction|Soziale Interaktion
softDeadline|Soft Deadline|Angestrebter Termin
soilConditions|Soil Conditions|Baugrundverhältnisse
sopReferences|SOP References|Verweise auf Standardarbeitsanweisungen
sortDirection|Sort Direction|Sortierrichtung
sortField|Sort Field|Sortierfeld
source|Source|Quelle
sourceId|Source|Quelle
sourceOrganization|Source Organisation|Herausgebende Organisation
sourceRelationshipId|Source Relationship|Ursprungsbeziehung
sourceSystem|Source System|Quellsystem
sources|Sources|Quellen
spaceRequirements|Space Requirements|Flächenanforderungen
spareParts|Spare Parts|Ersatzteile
staffJourney|Staff Journey|Wege des Personals
staffing|Staffing|Personalbedarf
stakeholderEngagementPlan|Stakeholder Engagement Plan|Beteiligungsplan
stakeholderFeedback|Stakeholder Feedback|Rückmeldungen der Beteiligten
stakeholderIds|Stakeholders|Beteiligte
stakeholderImpact|Stakeholder Impact|Auswirkungen auf Beteiligte
stakeholderType|Stakeholder Type|Art des Beteiligten
standard|Standard|Norm
standardRef|Standard Reference|Normverweis
standards|Standards|Normen
standbyPower|Standby Power|Ersatzstromversorgung
startDate|Start Date|Startdatum
startTime|Start Time|Startzeit
statement|Statement|Aussage
status|Status|Status
statusNotes|Status Notes|Statushinweise
statuses|Statuses|Status
steps|Steps|Schritte
storageClass|Storage Class|Lagerklasse
storageRequirementId|Storage Requirement|Lageranforderung
storedItem|Stored Goods|Lagergut
strategies|Strategies|Strategien
strength|Strength|Stärke
stressFactors|Stress Factors|Belastungsfaktoren
structuralSafety|Structural Safety|Standsicherheit
subjectId|Subject|Betreffende Entität
subjectIds|Subjects|Betroffene Personengruppen
subjectKind|Subject Kind|Art der betreffenden Entität
subtitle|Subtitle|Untertitel
success|Successful|Erfolgreich
successCriteria|Success Criteria|Erfolgskriterien
successMetrics|Success Metrics|Erfolgskennzahlen
successors|Successors|Nachfolger
summary|Summary|Zusammenfassung
supersededBy|Superseded By|Ersetzt durch
supersedes|Supersedes|Ersetzt
supervisionLevel|Supervision Level|Aufsichtsbedarf
supplier|Supplier|Lieferant
supplierRequirements|Supplier Requirements|Lieferantenanforderungen
supplyChain|Supply Chain|Lieferkette
surveillance|Surveillance|Überwachung
surveyId|Survey|Umfrage
surveyIds|Surveys|Umfragen
surveyStatus|Survey Status|Umfragestatus
surveyType|Survey Type|Umfrageart
symbolStandards|Symbol Standards|Piktogrammnormen|e.g. ISO 7010 safety signs.|z. B. Sicherheitszeichen nach ISO 7010.
system|System|System
tactileGuidance|Tactile Guidance|Taktiles Leitsystem
tactileRequired|Tactile Information Required|Taktile Information erforderlich
tag|Tag|Schlagwort
tagFilters|Tag Filters|Schlagwortfilter
tags|Tags|Schlagwörter
target|Target|Zielwert
SustainabilityRequirement.target|Target|Ziel
targetAudience|Target Audience|Zielgruppe
targetElementId|Target Element|Zielelement
targetId|Target|Ziel
targetLevel|Target Level|Zielniveau
targetValue|Target Value|Zielwert
technology|Technology|Technik
technologyControls|Technology Controls|Technische Maßnahmen
technologyProficiency|Technology Proficiency|Technikkompetenz
technologyReadiness|Technology Readiness|Technische Reife
temperatureRange|Temperature Range|Temperaturbereich
templateId|Template|Vorlage
templateType|Template Type|Vorlagenart
templates|Templates|Vorlagen
temporalPattern|Temporal Pattern|Zeitliches Muster
terminology|Terminology|Terminologie
testingRequirements|Testing Requirements|Prüfanforderungen
text|Text|Text
themes|Themes|Themen
thermalComfort|Thermal Comfort|Thermische Behaglichkeit
threat|Threat|Bedrohung
timeRestrictions|Time Restrictions|Zeitliche Beschränkungen
timeWindows|Time Windows|Zeitfenster
timestamp|Timestamp|Zeitstempel
timestamps|Timestamps|Zeitstempel
title|Title|Titel
toElementId|To Element|Zu Element
toId|To|Nach
tolerancePercent|Tolerance (%)|Toleranz (%)
toolVersion|Tool Version|Werkzeugversion
topic|Topic|Thema
traceLink|Trace Link|Nachverfolgungsbeziehung
traceLinks|Trace Links|Nachverfolgungsbeziehungen
tradeOffOptions|Trade-Off Options|Abwägungsoptionen
trafficIsolation|Traffic Isolation|Verkehrstrennung
trainingNeeds|Training Needs|Schulungsbedarf
trainingRequired|Training Required|Schulung erforderlich
trainingRequirements|Training Requirements|Schulungsanforderungen
trigger|Trigger|Auslöser
triggerEvents|Trigger Events|Auslösende Ereignisse
triggerIndicators|Early Warning Indicators|Frühwarnindikatoren
turnaroundTime|Turnaround Time|Bearbeitungszeit
turningCircleM|Manoeuvring Space (m)|Bewegungsfläche (m)|Size of the wheelchair manoeuvring space, e.g. 1.50 m per DIN 18040.|Größe der Bewegungsfläche für Rollstuhlnutzende, z. B. 1,50 m nach DIN 18040.
twoWay|Two-Way|Wechselseitig
unionConsiderations|Works Council Considerations|Belange der Arbeitnehmervertretung
unit|Unit|Einheit
unitCost|Unit Cost|Einheitspreis
universalDesignPrinciples|Universal Design Principles|Grundsätze des Universal Design
updateFrequency|Update Frequency|Aktualisierungshäufigkeit
updateSource|Update Source|Aktualisierungsquelle
updated|Updated|Geändert am
updatedBy|Updated By|Geändert von
uptimeTarget|Availability Target|Verfügbarkeitsziel
usageCount|Usage Count|Nutzungsanzahl
usageDuration|Usage Duration|Nutzungsdauer
usageFrequency|Usage Frequency|Nutzungshäufigkeit
useCount|Use Count|Verwendungsanzahl
userProfileIds|User Profiles|Nutzerprofile
utilities|Utilities|Medienversorgung
utilitiesAvailable|Available Utilities|Verfügbare Medienanschlüsse
utilityConnections|Utility Connections|Medienanschlüsse
utilitySource|Utility Source|Versorgungsquelle
validFrom|Valid From|Gültig ab
validUntil|Valid Until|Gültig bis
validated|Validated|Validiert
validatedBy|Validated By|Validiert von
validationDate|Validation Date|Validierungsdatum
validationNotes|Validation Notes|Validierungshinweise
validationStatus|Validation Status|Validierungsstatus
validationType|Validation Type|Validierungsart
validatorIds|Validators|Prüfende
value|Value|Wert
valueEngineeringNotes|Value Engineering Notes|Hinweise zur Wertanalyse
variables|Variables|Variablen
varianceNotes|Variance Notes|Abweichungshinweise
ventilation|Ventilation|Lüftung
ventilationStrategy|Ventilation Strategy|Lüftungskonzept
verificationMethod|Verification Method|Nachweisverfahren
verificationPlan|Verification Plan|Nachweisplan
verificationStatus|Verification Status|Nachweisstatus
version|Version|Version
versionFrom|From Version|Von Version
versionTo|To Version|Zu Version
views|Views|Ausblicke
visibilityLevel|Visibility Level|Einsehbarkeit
vision|Vision|Vision
visitorJourney|Visitor Journey|Besucherwege
visitorManagement|Visitor Management|Besuchermanagement
visitorPolicy|Visitor Policy|Besucherregelung
visualContrast|Visual Contrast|Visueller Kontrast
visualDemands|Visual Demands|Visuelle Anforderungen
visualPrivacy|Visual Privacy|Sichtschutz
volume|Volume|Aufkommen
ProgramElement.volume|Room Volume|Rauminhalt
volumeM3|Volume (m³)|Volumen (m³)
waiverApprover|Waiver Approver|Befreiung erteilt durch
waiverStatus|Waiver Status|Befreiungsstatus
waivers|Waivers|Befreiungen
wasteStrategy|Waste Strategy|Abfallkonzept
wasteStreams|Waste Streams|Abfallströme
waterStrategy|Water Strategy|Wasserkonzept
wcagConformance|WCAG Conformance|WCAG-Konformität
weatherWindows|Weather Windows|Wetterfenster
weight|Weight|Gewichtung
weightKg|Weight (kg)|Gewicht (kg)
weightedScore|Weighted Score|Gewichtete Punktzahl
wellnessPlugins|Wellbeing Programmes|Gesundheitsangebote
workHours|Working Hours|Arbeitszeiten
workPatterns|Work Patterns|Arbeitsmodelle
workaround|Workaround|Übergangslösung
workflowStep|Workflow Step|Workflow-Schritt
workflowSteps|Workflow Steps|Arbeitsschritte
workflowType|Workflow Type|Ablaufart
workshopId|Workshop|Workshop
workshopStatus|Workshop Status|Workshopstatus
workshopType|Workshop Type|Workshopart
zoneIds|Zones|Zonen
zoning|Zoning|Planungsrechtliche Einstufung|e.g. the type of building use per BauNVO.|z. B. die Art der baulichen Nutzung nach BauNVO.
"""

OPTION_TABLE = """
AccessLevel.public|Public|Öffentlich
AccessLevel.restricted|Restricted|Eingeschränkt
AccessLevel.controlled|Controlled|Kontrolliert
AccessLevel.private|Private|Privat
AccessLevel.secure|Secure|Gesichert
AccessLevel.emergencyOnly|Emergency Only|Nur im Notfall
AccessMode.unrestricted|Unrestricted|Uneingeschränkt
AccessMode.cardControlled|Card-Controlled|Kartengesteuert
AccessMode.biometric|Biometric|Biometrisch
AccessMode.keyed|Keyed|Mit Schlüssel
AccessMode.escortRequired|Escort Required|Nur mit Begleitung
AccessMode.timeRestricted|Time-Restricted|Zeitlich beschränkt
AccessMode.roleBased|Role-Based|Rollenbasiert
AccessMode.emergencyOnly|Emergency Only|Nur im Notfall
AdjacencyKind.required|Required|Erforderlich
AdjacencyKind.preferred|Preferred|Bevorzugt
AdjacencyKind.optional|Optional|Optional
AdjacencyKind.prohibited|Prohibited|Verboten
AnalysisKind.gap|Gap|Lücke
AnalysisKind.conflict|Conflict|Konflikt
AnalysisKind.dependency|Dependency|Abhängigkeit
AnalysisKind.capacity|Capacity|Kapazität
AnalysisKind.demand|Demand|Bedarf
AnalysisKind.utilization|Utilisation|Auslastung
AnalysisKind.workflow|Workflow|Arbeitsablauf
AnalysisKind.risk|Risk|Risiko
AnalysisKind.cost|Cost|Kosten
AnalysisKind.scenario|Scenario|Szenario
AnalysisKind.sensitivity|Sensitivity|Sensitivität
AnalysisKind.impact|Impact|Auswirkung
AnalysisKind.trend|Trend|Trend
AnalysisKind.requirementComparison|Requirement Comparison|Anforderungsvergleich
AnalysisKind.requirementClustering|Requirement Clustering|Anforderungsclusterung
AnalysisKind.requirementFiltering|Requirement Filtering|Anforderungsfilterung
AnalysisKind.requirementSorting|Requirement Sorting|Anforderungssortierung
AnalysisKind.requirementScoring|Requirement Scoring|Anforderungsbewertung
AnalysisKind.requirementWeighting|Requirement Weighting|Anforderungsgewichtung
AnalysisKind.relationshipAnalysis|Relationship Analysis|Beziehungsanalyse
AuditAction.created|Created|Erstellt
AuditAction.updated|Updated|Geändert
AuditAction.deleted|Deleted|Gelöscht
AuditAction.reviewed|Reviewed|Geprüft
AuditAction.approved|Approved|Freigegeben
AuditAction.rejected|Rejected|Abgelehnt
AuditAction.exported|Exported|Exportiert
AuditAction.imported|Imported|Importiert
AuditAction.merged|Merged|Zusammengeführt
AuditAction.archived|Archived|Archiviert
ConflictKind.adjacency|Adjacency|Adjazenz
ConflictKind.capacity|Capacity|Kapazität
ConflictKind.schedule|Schedule|Termine
ConflictKind.budget|Budget|Budget
ConflictKind.regulatory|Regulatory|Regulatorisch
ConflictKind.operational|Operational|Betrieblich
ConflictKind.environmental|Environmental|Umwelt
ConflictKind.security|Security|Sicherung
ConflictKind.priority|Priority|Priorität
ConnectionKind.direct|Direct|Direkt
ConnectionKind.indirect|Indirect|Indirekt
ConnectionKind.controlled|Controlled|Kontrolliert
ConnectionKind.sharedAccess|Shared Access|Gemeinsamer Zugang
ConnectionKind.none|None|Keine
CostBasis.capital|Capital|Investition
CostBasis.operational|Operational|Betrieb
CostBasis.lifecycle|Life Cycle|Lebenszyklus
CostBasis.replacement|Replacement|Ersatzinvestition
CostBasis.maintenance|Maintenance|Instandhaltung
DeliveryPhase.concept|Concept|Konzept
DeliveryPhase.schematic|Schematic Design|Vorplanung
DeliveryPhase.designDevelopment|Design Development|Entwurfsplanung
DeliveryPhase.constructionDocuments|Construction Documents|Ausführungsplanung
DeliveryPhase.procurement|Procurement|Vergabe
DeliveryPhase.construction|Construction|Bauausführung
DeliveryPhase.commissioning|Commissioning|Inbetriebnahme
DeliveryPhase.occupancy|Occupancy|Nutzung
EngagementLevel.unaware|Unaware|Nicht informiert
EngagementLevel.resistant|Resistant|Ablehnend
EngagementLevel.neutral|Neutral|Neutral
EngagementLevel.supportive|Supportive|Unterstützend
EngagementLevel.leading|Leading|Treibend
EnvironmentalParameter.temperature|Temperature|Temperatur
EnvironmentalParameter.humidity|Humidity|Luftfeuchte
EnvironmentalParameter.airQuality|Air Quality|Luftqualität
EnvironmentalParameter.lighting|Lighting|Beleuchtung
EnvironmentalParameter.acoustics|Acoustics|Akustik
EnvironmentalParameter.ventilation|Ventilation|Lüftung
EnvironmentalParameter.radiation|Radiation|Strahlung
EnvironmentalParameter.vibration|Vibration|Erschütterung
EnvironmentalParameter.pressure|Pressure|Druck
EnvironmentalParameter.iaq|Indoor Air Quality|Innenraumluftqualität
FlowDirection.oneWay|One-Way|Eine Richtung
FlowDirection.twoWay|Two-Way|Beide Richtungen
FlowDirection.bidirectionalPeak|Two-Way with Peaks|Beide Richtungen mit Spitzenlast
FlowDirection.restricted|Restricted|Eingeschränkt
FlowKind.people|People|Personen
FlowKind.material|Material|Material
FlowKind.information|Information|Information
FlowKind.service|Service|Service
FlowKind.equipment|Equipment|Geräte
FlowKind.waste|Waste|Abfall
FlowKind.emergency|Emergency|Notfall
FlowKind.vehicle|Vehicles|Fahrzeuge
FunctionKind.primary|Primary|Hauptfunktion
FunctionKind.secondary|Secondary|Nebenfunktion
FunctionKind.support|Support|Unterstützende Funktion
FunctionKind.administrative|Administrative|Verwaltung
FunctionKind.service|Service|Service
FunctionKind.technical|Technical|Technik
FunctionKind.public|Public|Öffentlich
FunctionKind.private|Private|Privat
FunctionKind.shared|Shared|Gemeinschaftlich
FunctionKind.restricted|Restricted|Eingeschränkt
FunctionKind.temporary|Temporary|Temporär
FunctionKind.future|Future|Zukünftig
FunctionKind.operational|Operational|Betrieb
FunctionKind.circulation|Circulation|Erschließung
HumanFactorAspect.ergonomics|Ergonomics|Ergonomie
HumanFactorAspect.cognition|Cognition|Kognition
HumanFactorAspect.sensory|Sensory|Sinneswahrnehmung
HumanFactorAspect.social|Social|Sozial
HumanFactorAspect.cultural|Cultural|Kulturell
HumanFactorAspect.behavioral|Behavioural|Verhalten
HumanFactorAspect.physical|Physical|Körperlich
HumanFactorAspect.psychological|Psychological|Psychologisch
HumanFactorAspect.fatigue|Fatigue|Ermüdung
HumanFactorAspect.stress|Stress|Stress
InfluenceLevel.low|Low|Gering
InfluenceLevel.medium|Medium|Mittel
InfluenceLevel.high|High|Hoch
InfluenceLevel.critical|Critical|Kritisch
IssueSeverity.cosmetic|Cosmetic|Kosmetisch
IssueSeverity.minor|Minor|Gering
IssueSeverity.major|Major|Erheblich
IssueSeverity.critical|Critical|Kritisch
IssueSeverity.blocker|Blocker|Blockierend
LifecycleStatus.draft|Draft|Entwurf
LifecycleStatus.proposed|Proposed|Vorgeschlagen
LifecycleStatus.underReview|Under Review|In Prüfung
LifecycleStatus.validated|Validated|Validiert
LifecycleStatus.approved|Approved|Freigegeben
LifecycleStatus.rejected|Rejected|Abgelehnt
LifecycleStatus.deferred|Deferred|Zurückgestellt
LifecycleStatus.superseded|Superseded|Ersetzt
LifecycleStatus.archived|Archived|Archiviert
LifecycleStatus.open|Open|Offen
LifecycleStatus.closed|Closed|Geschlossen
LifecycleStatus.atRisk|At Risk|Gefährdet
LifecycleStatus.blocked|Blocked|Blockiert
LifecycleStatus.inProgress|In Progress|In Bearbeitung
LifecycleStatus.complete|Complete|Abgeschlossen
Priority.mandatory|Mandatory|Zwingend
Priority.essential|Essential|Wesentlich
Priority.preferred|Preferred|Wünschenswert
Priority.optional|Optional|Optional
Priority.deferred|Deferred|Zurückgestellt
Priority.prohibited|Prohibited|Unzulässig
PrivacyKind.public|Public|Öffentlich
PrivacyKind.semiPublic|Semi-Public|Halböffentlich
PrivacyKind.semiPrivate|Semi-Private|Halbprivat
PrivacyKind.private|Private|Privat
PrivacyKind.confidential|Confidential|Vertraulich
PrivacyKind.restricted|Restricted|Eingeschränkt
PrivacyKind.anonymous|Anonymous|Anonym
ProgramElementKind.building|Building|Gebäude
ProgramElementKind.campus|Campus|Liegenschaft
ProgramElementKind.floor|Floor|Geschoss
ProgramElementKind.zone|Zone|Zone
ProgramElementKind.room|Room|Raum
ProgramElementKind.suite|Suite|Raumgruppe
ProgramElementKind.department|Department|Abteilung
ProgramElementKind.system|System|System
ProgramElementKind.circulation|Circulation Area|Verkehrsfläche
ProgramElementKind.support|Support Space|Nebenfläche
ProgramElementKind.outdoor|Outdoor Area|Außenfläche
ProgramElementKind.furnitureGroup|Furniture Group|Möblierungsgruppe
ProgramElementKind.other|Other|Sonstiges
RelationshipKind.contains|Contains|Enthält
RelationshipKind.serves|Serves|Versorgt
RelationshipKind.supports|Supports|Unterstützt
RelationshipKind.dependsOn|Depends On|Hängt ab von
RelationshipKind.conflictsWith|Conflicts With|Steht im Konflikt mit
RelationshipKind.equivalentTo|Equivalent To|Gleichwertig mit
RelationshipKind.adjacentTo|Adjacent To|Grenzt an
RelationshipKind.feeds|Feeds|Speist
RelationshipKind.receives|Receives|Empfängt
RelationshipKind.controls|Controls|Steuert
RelationshipKind.monitors|Monitors|Überwacht
RelationshipKind.functional|Functional|Funktional
RelationshipKind.operational|Operational|Betrieblich
RelationshipKind.organizational|Organisational|Organisatorisch
RelationshipKind.user|User|Nutzerbezogen
RelationshipKind.service|Service|Service
RelationshipKind.information|Information|Information
RelationshipKind.access|Access|Zugang
RelationshipKind.security|Security|Sicherung
RelationshipKind.supervision|Supervision|Aufsicht
RelationshipKind.communication|Communication|Kommunikation
RelationshipKind.dependency|Dependency|Abhängigkeit
RelationshipKind.sequential|Sequential|Sequenziell
RelationshipKind.sharedResource|Shared Resource|Gemeinsame Ressource
ReportKind.executiveSummary|Executive Summary|Kurzfassung
ReportKind.programOverview|Program Overview|Programmübersicht
ReportKind.stakeholderSummary|Stakeholder Summary|Beteiligtenübersicht
ReportKind.requirementsMatrix|Requirements Matrix|Anforderungsmatrix
ReportKind.adjacencyMatrix|Adjacency Matrix|Adjazenzmatrix
ReportKind.gapAnalysis|Gap Analysis|Lückenanalyse
ReportKind.riskRegister|Risk Register|Risikoregister
ReportKind.decisionLog|Decision Log|Entscheidungsprotokoll
ReportKind.validationSummary|Validation Summary|Validierungsübersicht
ReportKind.recommendation|Recommendation|Empfehlung
ReportKind.userSummary|User Summary|Nutzerübersicht
ReportKind.functionalSummary|Functional Summary|Funktionsübersicht
ReportKind.capacitySummary|Capacity Summary|Kapazitätsübersicht
ReportKind.workflowSummary|Workflow Summary|Arbeitsablaufübersicht
ReportKind.complianceSummary|Compliance Summary|Compliance-Übersicht
ReportKind.costSummary|Cost Summary|Kostenübersicht
ReportKind.scheduleSummary|Schedule Summary|Terminübersicht
ReportKind.changeSummary|Change Summary|Änderungsübersicht
ReportKind.openIssueSummary|Open Issue Summary|Übersicht offener Punkte
ReportKind.prioritySummary|Priority Summary|Prioritätenübersicht
ReportKind.scenarioSummary|Scenario Summary|Szenarienübersicht
RequirementKind.functional|Functional|Funktional
RequirementKind.spatial|Spatial|Räumlich
RequirementKind.performance|Performance|Leistung
RequirementKind.regulatory|Regulatory|Regulatorisch
RequirementKind.operational|Operational|Betrieblich
RequirementKind.technical|Technical|Technisch
RequirementKind.aesthetic|Aesthetic|Gestalterisch
RequirementKind.sustainability|Sustainability|Nachhaltigkeit
RiskLevel.negligible|Negligible|Vernachlässigbar
RiskLevel.low|Low|Gering
RiskLevel.medium|Medium|Mittel
RiskLevel.high|High|Hoch
RiskLevel.critical|Critical|Kritisch
SafetyDomain.lifeSafety|Life Safety|Personenschutz
SafetyDomain.occupationalHealth|Occupational Health|Arbeitsschutz
SafetyDomain.fire|Fire|Brandschutz
SafetyDomain.structural|Structural|Standsicherheit
SafetyDomain.electrical|Electrical|Elektrische Sicherheit
SafetyDomain.chemical|Chemical|Gefahrstoffe
SafetyDomain.radiation|Radiation|Strahlenschutz
SafetyDomain.ergonomics|Ergonomics|Ergonomie
SafetyDomain.biological|Biological|Biologische Arbeitsstoffe
SafetyDomain.environmental|Environmental|Umweltschutz
SecurityControlKind.accessControl|Access Control|Zutrittskontrolle
SecurityControlKind.surveillance|Surveillance|Überwachung
SecurityControlKind.perimeter|Perimeter|Perimeterschutz
SecurityControlKind.cyber|Cyber|IT-Sicherheit
SecurityControlKind.personnel|Personnel|Personelle Sicherung
SecurityControlKind.information|Information|Informationssicherheit
SecurityControlKind.physical|Physical|Mechanische Sicherung
SecurityControlKind.procedural|Procedural|Organisatorische Sicherung
SecurityControlKind.screening|Screening|Personen- und Gepäckkontrolle
SecurityControlKind.keyManagement|Key Management|Schlüsselverwaltung
SeparationKind.acoustic|Acoustic|Schallschutz
SeparationKind.visual|Visual|Sichtschutz
SeparationKind.security|Security|Sicherung
SeparationKind.olfactory|Olfactory|Geruchsschutz
SeparationKind.thermal|Thermal|Wärmeschutz
SeparationKind.fire|Fire|Brandschutz
SeparationKind.hygienic|Hygienic|Hygiene
SeparationKind.circulation|Circulation|Verkehrstrennung
SeparationKind.operational|Operational|Betriebliche Trennung
SeparationKind.infectionControl|Infection Control|Infektionsschutz
StorageClass.general|General|Allgemein
StorageClass.secure|Secure|Gesichert
StorageClass.climateControlled|Climate-Controlled|Klimatisiert
StorageClass.hazardous|Hazardous Substances|Gefahrstoffe
StorageClass.archive|Archive|Archiv
StorageClass.mobile|Mobile|Mobil
StorageClass.fixed|Fixed|Ortsfest
StorageClass.shared|Shared|Gemeinsam genutzt
StorageClass.coldChain|Cold Chain|Kühlkette
StorageClass.flammable|Flammable|Entzündbare Stoffe
TraceKind.objectiveToRequirement|Objective → Requirement|Ziel → Anforderung
TraceKind.stakeholderToRequirement|Stakeholder → Requirement|Beteiligter → Anforderung
TraceKind.userToActivity|User → Activity|Nutzer → Aktivität
TraceKind.activityToFunction|Activity → Function|Aktivität → Funktion
TraceKind.functionToProgramElement|Function → Program Element|Funktion → Programmelement
TraceKind.requirementToDecision|Requirement → Decision|Anforderung → Entscheidung
TraceKind.requirementToRisk|Requirement → Risk|Anforderung → Risiko
TraceKind.requirementToStandard|Requirement → Standard|Anforderung → Norm
TraceKind.requirementToValidation|Requirement → Validation|Anforderung → Validierung
TraceKind.requirementToApproval|Requirement → Approval|Anforderung → Freigabe
TraceKind.requirementToChange|Requirement → Change|Anforderung → Änderung
TraceKind.equipmentToActivity|Equipment → Activity|Ausstattung → Aktivität
TraceKind.processToResource|Process → Resource|Prozess → Ressource
TraceKind.constraintToImpact|Constraint → Impact|Randbedingung → Auswirkung
TraceKind.scenarioToDecision|Scenario → Decision|Szenario → Entscheidung
TraceKind.issueToAction|Issue → Action|Offener Punkt → Maßnahme
TraceKind.actionToOwner|Action → Owner|Maßnahme → Verantwortliche
TraceKind.decisionToOutcome|Decision → Outcome|Entscheidung → Ergebnis
TraceKind.versionToChange|Version → Change|Version → Änderung
TraceKind.fullAuditTrail|Full Audit Trail|Vollständiger Prüfpfad
UserCategory.primary|Primary Users|Hauptnutzer
UserCategory.secondary|Secondary Users|Nebennutzer
UserCategory.occasional|Occasional Users|Gelegenheitsnutzer
UserCategory.service|Service Staff|Servicepersonal
UserCategory.visitor|Visitors|Besucher
UserCategory.staff|Staff|Personal
UserCategory.public|General Public|Öffentlichkeit
ValidationStatus.pending|Pending|Ausstehend
ValidationStatus.passed|Passed|Bestanden
ValidationStatus.failed|Failed|Nicht bestanden
ValidationStatus.waived|Waived|Befreit
ValidationStatus.deferred|Deferred|Zurückgestellt
"""

KIND_LABELS = {
    "AdjacencyKind": ("Adjacency Kind", "Adjazenzart"),
    "AnalysisKind": ("Analysis Kind", "Analyseart"),
    "ConflictKind": ("Conflict Kind", "Konfliktart"),
    "FlowKind": ("Flow Kind", "Flussart"),
    "FunctionKind": ("Function Kind", "Funktionsart"),
    "ProgramElementKind": ("Element Kind", "Elementart"),
    "RelationshipKind": ("Relationship Kind", "Beziehungsart"),
    "ReportKind": ("Report Kind", "Berichtsart"),
    "RequirementKind": ("Requirement Kind", "Anforderungsart"),
    "TraceKind": ("Trace Kind", "Art der Nachverfolgungsbeziehung"),
}

REFERENCES = {
    "ownerId": "stakeholder", "authorityId": "stakeholder", "consultantIds": "stakeholder", "participantIds": "stakeholder",
    "createdBy": "stakeholder", "updatedBy": "stakeholder", "resourceIds": "resource", "subjectIds": ["userProfile", "stakeholder"],
    "zoneIds": "programElement", "elementIds": "programElement", "routeIds": ["flowRequirement", "programElement"],
    "userProfileIds": "userProfile", "adjacentActivities": "activity", "equipmentIds": "equipment", "functionIds": "function",
    "conflictIds": "conflict", "elementAId": "programElement", "elementBId": "programElement", "sourceRelationshipId": "relationship",
    "inputEntityIds": "entity", "reportId": "reportRecord", "runBy": "stakeholder", "scenarioId": "scenario", "approverIds": "stakeholder",
    "delegationChain": "stakeholder", "notificationList": "stakeholder", "relatedChangeId": "changeRecord", "relatedDecisionId": "decision",
    "subjectId": "entity", "authorIds": "stakeholder", "distributionList": "stakeholder", "relatedEntityIds": "entity",
    "reviewerIds": "stakeholder", "supersedes": "document", "linkedRequirementIds": "requirement", "linkedRiskIds": "risk",
    "validatedBy": "stakeholder", "actorId": "stakeholder", "changeRecordId": "changeRecord", "knowledgeId": "knowledgeRecord",
    "relatedRequirementIds": "requirement", "approvedBy": "stakeholder", "auditEventIds": "auditEvent", "impactedEntityIds": "entity",
    "requestedBy": "stakeholder", "decisionIds": "decision", "documentIds": "document", "facilitatorId": "stakeholder",
    "issueIds": "issue", "participants": "stakeholder", "surveyId": "survey", "workshopId": "workshop",
    "audienceIds": ["stakeholder", "userProfile"], "templates": "templateRecord", "affectedEntityIds": "entity",
    "auditorId": "stakeholder", "decisionId": "decision", "entityAId": "entity", "entityBId": "entity", "relatedRiskIds": "risk",
    "requirementIds": "requirement", "stakeholderIds": "stakeholder", "escalationContactId": "stakeholder",
    "relatedDecisionIds": "decision", "waiverApprover": "stakeholder", "benchmarkRef": "benchmarkRecord", "artifactRefs": "document",
    "consultedIds": "stakeholder", "decisionMakerIds": "stakeholder", "informedIds": "stakeholder",
    "impactedElementIds": "programElement", "impactedRequirementIds": "requirement", "meetingRef": "meetingRecord",
    "optionsConsidered": "optionEvaluation", "selectedOptionId": "optionEvaluation", "riskIds": "risk", "activityIds": "activity",
    "activityLinkIds": "trace", "futureFunctionIds": "function", "fromElementId": "programElement", "toElementId": "programElement",
    "processId": "process", "processIds": "process", "hierarchyParentId": "function", "ownerStakeholderId": "stakeholder",
    "policyOwnershipId": "stakeholder", "requirementOwnershipId": "stakeholder", "riskOwnershipId": "stakeholder",
    "decisionPoints": "decision", "expansionElementIds": "programElement", "riskFactors": "risk", "scenarioIds": "scenario",
    "assigneeId": "stakeholder", "reporterId": "stakeholder", "attachments": "document", "approvalIds": "approvalRecord",
    "attendeeIds": "stakeholder", "chairId": "stakeholder", "decisionsMade": "decision", "criteriaIds": "performanceCriterion",
    "evaluatorIds": "stakeholder", "growthPlanId": "growthPlan", "serviceRequirementIds": "serviceRequirement", "conflicts": "conflict",
    "actors": ["stakeholder", "userProfile"], "adjacencyIds": "adjacency", "adjacencyPreferences": "programElement",
    "parentId": "programElement", "quantityIds": "quantityRequirement", "responsibleParty": "stakeholder",
    "targetElementId": "programElement", "consultantRefs": "stakeholder", "sourceId": "entity", "targetId": "entity",
    "analysisIds": "analysisRecord", "approverId": "stakeholder", "generatedBy": "stakeholder", "templateId": "templateRecord",
    "childRequirementIds": "requirement", "parentRequirementId": "requirement", "supersededBy": "requirement",
    "infrastructureIds": "infrastructureRequirement", "storageRequirementId": "storageRequirement",
    "affectedElementIds": "programElement", "affectedRequirementIds": "requirement", "relatedConflictIds": "conflict",
    "affectedUserIds": "userProfile", "optionIds": "optionEvaluation", "predecessors": "scheduleRequirement",
    "successors": "scheduleRequirement", "ownerIds": "stakeholder", "assetIds": "entity", "customerProfiles": "userProfile",
    "delegatedTo": "stakeholder", "representativeOf": "stakeholder", "changedBy": "stakeholder", "milestoneId": "scheduleRequirement",
    "relatedIssueIds": "issue", "analysisId": "analysisRecord", "targetAudience": ["stakeholder", "userProfile"],
    "authorId": "stakeholder", "benchmarkIds": "benchmarkRecord", "relatedKnowledgeIds": "knowledgeRecord", "fromId": "entity",
    "toId": "entity", "validatorIds": "stakeholder", "decisions": "decision", "issues": "issue", "surveyIds": "survey",
    "Function.dependencies": "function", "OptionEvaluation.dependencies": "entity", "PriorityRecord.dependencies": "entity",
    "Process.dependencies": "process", "ScheduleRequirement.dependencies": "scheduleRequirement",
    "ServiceRequirement.dependencies": "serviceRequirement",
}

NUMERIC = {
    "clearHeightM": dict(unit="m", step=0.01, precision=2), "clearWidthM": dict(unit="m", step=0.01, precision=2),
    "distanceConstraintM": dict(unit="m", step=0.1, precision=2), "distanceMaxM": dict(unit="m", step=0.1, precision=2),
    "distanceMinM": dict(unit="m", step=0.1, precision=2), "elevationM": dict(unit="m", step=0.01, precision=2),
    "maxHeightM": dict(unit="m", step=0.1, precision=2), "maximumSignageDistanceM": dict(unit="m", step=0.5, precision=1),
    "turningCircleM": dict(unit="m", step=0.05, precision=2), "volumeM3": dict(unit="m³", step=0.1, precision=2),
    "weightKg": dict(unit="kg", step=1, precision=1), "powerKw": dict(unit="kW", step=0.1, precision=2),
    "noiseLevelDb": dict(unit="dB", step=1, precision=1), "durationMs": dict(unit="ms"),
    "progressPercent": dict(widget="slider", unit="%", step=1, precision=0, softMin=0, softMax=100),
    "contingencyPercent": dict(unit="%", step=0.5, precision=1), "tolerancePercent": dict(unit="%", step=0.5, precision=1),
    "contingencyDays": dict(unit="d"), "floatDays": dict(unit="d"), "horizonYears": dict(unit="a"), "lifecycleYears": dict(unit="a"),
    "latitude": dict(unit="°", step=0.000001, precision=6), "longitude": dict(unit="°", step=0.000001, precision=6),
    "areaDelta": dict(unit="m²", step=1, precision=2), "headcountDelta": dict(step=1, precision=0),
    "diversityFactor": dict(step=0.01, precision=2), "peakFactor": dict(step=0.01, precision=2), "growthFactor": dict(step=0.01, precision=2),
    "sharingRatio": dict(step=0.01, precision=2), "maxCoverage": dict(step=0.01, precision=2), "growthRate": dict(step=0.01, precision=2),
    "escalationRate": dict(step=0.01, precision=2), "responseRate": dict(step=0.01, precision=2), "uptimeTarget": dict(step=0.01, precision=2),
    "probability": dict(step=0.01, precision=2), "weight": dict(step=0.1, precision=2), "score": dict(step=0.1, precision=2),
    "strength": dict(step=0.1, precision=2), "riskScore": dict(step=0.1, precision=2), "weightedScore": dict(step=0.1, precision=2),
    "rampSlope": dict(step=0.1, precision=2), "embodiedCarbon": dict(step=1, precision=1), "operationalCarbon": dict(step=1, precision=1),
}
BOUNDS = {"latitude": {"minimum": -90, "maximum": 90}, "longitude": {"minimum": -180, "maximum": 180}}
MONEY = {"amount", "unitCost", "costPerUnit", "costEstimate", "costImpact", "costDelta", "replacementCost", "lifecycleCost", "budget", "budgetEnvelope", "costOfChange"}
DATES = {
    "approvalDate", "auditDate", "changeDate", "changedAt", "closeDate", "created", "dateFrom", "dateTo", "decisionDate", "detectionDate", "dueDate",
    "effectiveDate", "effectiveFrom", "effectiveUntil", "endDate", "endTime", "evaluationDate", "expirationDate", "expiryDate", "followUpDate",
    "generatedAt", "issueDate", "lastApplied", "lastReviewed", "lastUsed", "lastVerified", "launchDate", "nextReview", "nextReviewDate",
    "resolvedDate", "resubmissionDate", "retentionUntil", "reviewDate", "revisionDate", "runAt", "scheduledDate", "scheduledEnd",
    "scheduledStart", "startDate", "startTime", "timestamp", "updated", "validFrom", "validUntil", "validationDate", "hardDeadline", "softDeadline",
}
IDENTITY = {"id", "name", "code", "title", "subtitle", "schema", "documentId", "revision", "locale", "framework", "siteName", "filterName", "benchmarkName", "optionName", "serviceName", "version", "label", "clientName"}
STATUS_ENUMS = {"LifecycleStatus", "Priority", "ValidationStatus", "RiskLevel", "IssueSeverity", "InfluenceLevel", "EngagementLevel"}
BLOCKS = {"TextField": "narrative", "Ownership": "ownership", "TimestampMeta": "audit", "TaggedNote": "notes", "QuantitySpec": "quantities", "TraceLink": "links"}
MULTILINE = {("TextField", "text"), ("TaggedNote", "text")}
ISO_DATE = L("Date or date-time per ISO 8601.", "Datum oder Zeitpunkt nach ISO 8601.")


def parse_table(text, width):
    table = {}
    for line in text.strip().split("\n"):
        cells = line.split("|")
        if len(cells) not in width:
            raise SystemExit("🏛️ malformed table line: %s" % line)
        key = cells[0]
        if key in table:
            raise SystemExit("🏛️ duplicate table key %s" % key)
        table[key] = cells[1:]
    return table


LABELS = parse_table(LABEL_TABLE, (3, 5))
OPTION_CELLS = parse_table(OPTION_TABLE, (3,))
OPTIONS = {}
for key, (en, de) in OPTION_CELLS.items():
    enum, value = key.split(".", 1)
    OPTIONS.setdefault(enum, {})[value] = L(en, de)
for enum, values in OPTIONS.items():
    if list(values) != ENUMS[enum]:
        raise SystemExit("🏛️ option labels of %s disagree with Rust: %s vs %s" % (enum, list(values), ENUMS[enum]))
USED = set()
MISSING = set()
#endregion 🔖️Vocabulary


#region 🔖️Annotation
def label_of(struct, key):
    for name in ("%s.%s" % (struct, key), key):
        if name in LABELS:
            USED.add(name)
            cells = LABELS[name]
            return L(cells[0], cells[1]), (L(cells[2], cells[3]) if len(cells) == 4 else None)
    MISSING.add("%s.%s" % (struct, key))
    return L(key, key), None


def reference_kinds(struct, key):
    kinds = REFERENCES.get("%s.%s" % (struct, key), REFERENCES.get(key))
    if kinds is None:
        raise SystemExit("🏛️ no reference kind for %s.%s" % (struct, key))
    return kinds


def reference(kinds):
    return {"kind": kinds, "domain": DOMAIN, "granularity": GRANULARITY}


def concrete(node):
    if "anyOf" in node:
        branches = [branch for branch in node["anyOf"] if branch != {"type": "null"}]
        return branches[0]
    return node


def group_of(struct, key, base, is_reference):
    if struct == "Ownership" or key == "ownership":
        return "ownership"
    if struct == "TimestampMeta" or key in ("timestamps", "notes", "tags"):
        return "audit"
    if key in IDENTITY and not is_reference:
        return "identity"
    if is_reference:
        return "links"
    if base in BLOCKS:
        return BLOCKS[base]
    if base in ENUMS:
        return "status" if base in STATUS_ENUMS else "classification"
    if key in DATES:
        return "schedule"
    if base in UNSIGNED | SIGNED | FLOATS:
        return "metrics"
    if base == "bool":
        return "flags"
    return "details"


def annotate_object(node, struct, context):
    for index, (key, value) in enumerate(list(node["properties"].items())):
        node["properties"][key] = annotate_field(value, struct, key, (index + 1) * 10, context)
    return node


def identity(struct, key, context, order):
    label, description = label_of(struct, key)
    if struct == context["definition"] and context["verb"] == "replace":
        en, de = ENTITIES[context["entity"]]
        return ui(widget="reference", role="target", label=label, description=L("The %s this record replaces." % en.lower(), "Eintrag vom Typ „%s“, den dieser Datensatz ersetzt." % de), ref=reference(context["entity"]), group="identity", order=order)
    if struct == context["definition"] and context["verb"] in ("create", "connect"):
        description = L("Unique ID of the new entry.", "Eindeutige ID des neuen Eintrags.")
    return ui(widget="text", role="value", label=label, description=description, group="identity", order=order)


def annotate_field(node, struct, key, order, context):
    out = {name: value for name, value in node.items() if name != "x-semio-ui"}
    inner = concrete(out)
    field = rust_field(struct, key)
    base, option, vec = base_type(field["type"])
    label, description = label_of(struct, key)
    if base == "EntityId" and key == "id" and not vec:
        out["x-semio-ui"] = identity(struct, key, context, order)
        return out
    if base == "EntityId":
        out["x-semio-ui"] = ui(widget="reference", role="target", label=label, description=description, ref=reference(reference_kinds(struct, key)), group="links", order=order)
        return out
    group = group_of(struct, key, base, False)
    if base in ENUMS:
        options = OPTIONS[base]
        if key == "kind" and base in KIND_LABELS:
            label = L(*KIND_LABELS[base])
        if vec:
            inner["items"] = dict(inner["items"], **{"x-semio-ui": {"options": options}})
            out["x-semio-ui"] = ui(role="value", label=label, description=description, group=group, order=order)
        else:
            widget = "segmented" if len(options) <= 4 else "select"
            out["x-semio-ui"] = ui(widget=widget, role="value", label=label, description=description, options=options, group=group, order=order)
        return out
    if base in STRUCTS and base not in ("EntityId",):
        if vec:
            annotate_object(inner["items"], base, context)
        else:
            annotate_object(inner, base, context)
        out["x-semio-ui"] = ui(role="value", label=label, description=description, group=group, order=order)
        return out
    if base == "String":
        if vec:
            out["x-semio-ui"] = ui(role="value", label=label, description=description, group=group, order=order)
        else:
            widget = "multiline" if (struct, key) in MULTILINE else "text"
            if description is None and key in DATES:
                description = ISO_DATE
            out["x-semio-ui"] = ui(widget=widget, role="value", label=label, description=description, group=group, order=order)
        return out
    if base == "bool":
        out["x-semio-ui"] = ui(widget="toggle", role="value", label=label, description=description, group=group, order=order)
        return out
    if base in UNSIGNED | SIGNED | FLOATS:
        if vec:
            out["x-semio-ui"] = ui(role="value", label=label, description=description, group=group, order=order)
            return out
        facets = dict(NUMERIC.get(key, {}))
        widget = facets.pop("widget", "stepper")
        if base in UNSIGNED | SIGNED:
            facets.setdefault("step", 1)
            facets.setdefault("precision", 0)
        elif key in MONEY:
            facets.setdefault("step", 1)
            facets.setdefault("precision", 2)
        else:
            facets.setdefault("step", 0.1)
            facets.setdefault("precision", 2)
        if key in BOUNDS:
            inner.update(BOUNDS[key])
        out["x-semio-ui"] = ui(widget=widget, role="value", label=label, description=description, group=group, order=order, **facets)
        return out
    raise SystemExit("🏛️ %s.%s has an unhandled Rust type %s" % (struct, key, field["type"]))
#endregion 🔖️Annotation


#region 🔖️Leaves
VERBS = ("create", "replace", "rename", "delete", "connect", "disconnect")
RENAMED = {"newName": None, "newTitle": ("meta", "New Title", "Neuer Titel", "New title of the program.", "Neuer Titel des Programms."), "newCode": ("project", "New Project Code", "Neue Projektkennung", "New code of the project.", "Neue Kennung des Projekts."), "newFramework": ("governance", "New Governance Framework", "Neues Governance-Rahmenwerk", "e.g. ISO 9001 or ISO 41001.", "z. B. ISO 9001 oder ISO 41001.")}
FACETS = {"newMeta": "meta", "newProject": "project", "newGovernance": "governance"}
FACET_DESCRIPTIONS = {
    "meta": L("The complete program metadata; replaces the current metadata.", "Vollständige Programm-Metadaten; ersetzen die bisherigen."),
    "project": L("The complete project definition; replaces the current one.", "Vollständige Projektdefinition; ersetzt die bisherige."),
    "governance": L("The complete governance; replaces the current one.", "Vollständige Governance; ersetzt die bisherige."),
}


def camel(kebab):
    head, *rest = kebab.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


def pascal(kebab):
    return "".join(word[:1].upper() + word[1:] for word in kebab.split("-"))


def leaf_kind(schema):
    return schema["$id"].split("/mutation/")[1].split("/")[0]


def record_description(verb, entity):
    en, de = ENTITIES[entity]
    if verb in ("create", "connect"):
        return L("The new %s with all of its fields." % en.lower(), "Neuer Eintrag vom Typ „%s“ mit allen Feldern." % de)
    return L("The complete %s; it replaces the entry with the same ID." % en.lower(), "Vollständiger Eintrag vom Typ „%s“; ersetzt den Eintrag mit derselben ID." % de)


def annotate_leaf(schema):
    kind = leaf_kind(schema)
    verb = next(prefix for prefix in VERBS if kind.startswith(prefix + "-"))
    entity = camel(kind[len(verb) + 1:])
    en, de = ENTITIES[entity]
    properties = {}
    for index, (key, node) in enumerate(schema["properties"].items()):
        order = (index + 1) * 10
        if key == "id":
            action = {"rename": ("rename", "Umzubenennender"), "delete": ("delete", "Zu löschender"), "disconnect": ("remove", "Zu entfernender")}[verb]
            properties[key] = {"type": "string", "x-semio-ui": ui(widget="reference", role="target", label=L(en, de), description=L("The %s to %s." % (en.lower(), action[0]), "%s Eintrag vom Typ „%s“." % (action[1], de)), ref=reference(entity), group="target", order=order)}
        elif key in RENAMED:
            body = {name: value for name, value in node.items() if name != "x-semio-ui"}
            if key == "newName":
                annotation = ui(widget="text", role="value", label=L("New Name", "Neuer Name"), description=L("New name of the %s." % en.lower(), "Neuer Name des Eintrags vom Typ „%s“." % de), group="identity", order=order)
            else:
                _, label_en, label_de, text_en, text_de = RENAMED[key]
                annotation = ui(widget="text", role="value", label=L(label_en, label_de), description=L(text_en, text_de), group="identity", order=order)
            properties[key] = dict(body, **{"x-semio-ui": annotation})
        else:
            facet = FACETS.get(key)
            target = facet or entity
            if facet is None and key != entity:
                raise SystemExit("🏛️ %s: unexpected input %s" % (kind, key))
            definition = DEFINITIONS.get(target, pascal(kind[len(verb) + 1:]))
            context = {"verb": "facet" if facet else "create" if verb == "connect" else verb, "entity": target, "definition": definition}
            record = annotate_object(truth_object(definition), definition, context)
            label_en, label_de = ENTITIES[target]
            description = FACET_DESCRIPTIONS[facet] if facet else record_description(verb, target)
            record["x-semio-ui"] = ui(role="value", label=L(label_en, label_de), description=description, group="record", order=order)
            properties[key] = record
    schema["properties"] = properties
    return schema


def write(path, schema):
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(schema, indent=2, ensure_ascii=False) + "\n")


def annotate_editor():
    config = json.load(open(CONFIG_LEAF, encoding="utf-8"))
    fields = config["properties"]["config"]["properties"]
    hidden = L("Machine-written cache; not edited by hand.", "Maschinell geschriebener Zwischenspeicher; wird nicht von Hand bearbeitet.")
    annotations = {
        "searchQuery": ui(widget="text", role="value", label=L("Search Query", "Suchanfrage"), group="search", order=10),
        "searchHistoryJson": ui(widget="hidden", role="value", label=L("Search History (JSON)", "Suchverlauf (JSON)"), description=hidden, group="search", order=20),
        "lastResultJson": ui(widget="hidden", role="value", label=L("Last Result (JSON)", "Letztes Ergebnis (JSON)"), description=hidden, group="results", order=30),
        "lastAnalysisJson": ui(widget="hidden", role="value", label=L("Last Analysis (JSON)", "Letzte Analyse (JSON)"), description=hidden, group="results", order=40),
    }
    for key in fields:
        fields[key] = dict({name: value for name, value in fields[key].items() if name != "x-semio-ui"}, **{"x-semio-ui": annotations[key]})
    config["properties"]["config"]["x-semio-ui"] = ui(role="value", label=L("Configuration", "Konfiguration"), description=L("The complete architect editor configuration.", "Vollständige Konfiguration des Architect-Editors."), group="record", order=10)
    write(CONFIG_LEAF, config)
    presence = json.load(open(PRESENCE_LEAF, encoding="utf-8"))
    fields = presence["properties"]["presence"]["properties"]
    annotations = {
        "activeRegister": ui(widget="text", role="value", label=L("Active Register", "Aktives Register"), description=L("Register shown in the register window, e.g. elements.", "Im Registerfenster angezeigtes Register, z. B. elements."), group="view", order=10),
        "adjacencyKindFilter": ui(widget="segmented", role="value", label=L("Adjacency Filter", "Adjazenzfilter"), options=OPTIONS["AdjacencyKind"], group="view", order=20),
        "graphCameraX": ui(widget="stepper", role="value", label=L("Graph Camera X", "Graphkamera X"), step=1, precision=1, group="camera", order=30),
        "graphCameraY": ui(widget="stepper", role="value", label=L("Graph Camera Y", "Graphkamera Y"), step=1, precision=1, group="camera", order=40),
        "graphCameraZoom": ui(widget="slider", role="value", label=L("Graph Zoom", "Graph-Zoom"), step=0.01, precision=2, softMin=0.1, softMax=10, scale="log", snaps=[0.25, 0.5, 1, 2, 4], group="camera", order=50),
    }
    for key in fields:
        fields[key] = dict({name: value for name, value in fields[key].items() if name != "x-semio-ui"}, **{"x-semio-ui": annotations[key]})
    presence["properties"]["presence"]["x-semio-ui"] = ui(role="value", label=L("Presence", "Präsenz"), description=L("The complete ephemeral view state of one architect user.", "Vollständiger flüchtiger Ansichtszustand eines Architect-Nutzers."), group="record", order=10)
    write(PRESENCE_LEAF, presence)


def main():
    annotated = []
    for entity_dir in sorted(os.listdir(LEAVES)):
        base = os.path.join(LEAVES, entity_dir)
        if not os.path.isdir(base):
            continue
        for verb_dir in sorted(os.listdir(base)):
            path = os.path.join(base, verb_dir, "🧬️schema", "🔣️.json")
            if os.path.exists(path):
                annotated.append((path, annotate_leaf(json.load(open(path, encoding="utf-8")))))
    if MISSING:
        raise SystemExit("🏛️ no label for: %s" % ", ".join(sorted(MISSING)))
    for path, schema in annotated:
        write(path, schema)
    annotate_editor()
    count = len(annotated)
    unused = sorted(set(LABELS) - USED)
    if unused:
        print("🏛️ unused labels: %s" % ", ".join(unused))
    print("annotated %d program leaf schemas + 2 editor leaves" % count)


if __name__ == "__main__":
    sys.exit(main())
#endregion 🔖️Leaves
