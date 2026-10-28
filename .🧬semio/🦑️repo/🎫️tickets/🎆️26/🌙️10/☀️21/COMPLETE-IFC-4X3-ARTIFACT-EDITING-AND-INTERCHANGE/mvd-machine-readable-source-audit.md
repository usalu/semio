# IFC4x3 Machine-Readable MVD Availability Audit

Primary-source discovery, not production implementation or fullview proof. Authored mvd-machine-source-census.py pins candidate XML and exact release root listing; generated/mvd-machine holds original XML, manifest and selected alignment template rule roster. No inference of XML absence from missing HTML links.

## Historical machine-readable candidate actually found

The publicly cited buildingSMART/IFC4.3.x-output repository now redirects via GitHub API to buildingSMART/IFC-translations (default branch input). Master no longer resolves. API history for IFC4.3.mvdxml identifies removal commit06d49f3f22f985221cb4773e18c3906f53f0fd9b and parent d7465622831898747817ed4767332f432fa55be9. Retrieved immutable [historical mvdXML](https://raw.githubusercontent.com/buildingSMART/IFC-translations/d7465622831898747817ed4767332f432fa55be9/IFC4.3.mvdxml),2523281bytes,SHA256e001c6b2c859e2eee4f34982dda06c2dffe02e6a8ac15847f3c1ae775090f01e.

Parsed XML contains314 ConceptTemplates,2063 AttributeRule,2276 EntityRule,2538 TemplateRule,947 TemplateRules,366 ConceptRoot and1205 Concept nodes. Crucially it has only one ModelView: General Usage, versionDEV,statussample,applicableSchema IFC4X3; document root also statussample. It is not a frozen ADD2 ReferenceView or Alignment-basedView selection and cannot be promoted to released normative constraints. Its machine-readable entity/attribute traversal and template predicates are useful discovery/test inputs if each proposed clause is reconciled with immutable released ADD2 sources and explicit applicability. Counts differ from pinned released312concepts/1188usage rows, further preventing silent substitution.

## Published status versus executable artifact

The [official MVD database](https://technical.buildingsmart.org/standards/ifc/mvd/mvd-database/) search result currently lists IFC4.3 ADD2 Alignment based View Final with content integrated in IFC documentation. Its IFC4 ADD2TC1 ReferenceView1.2 row links actual mvdXML. These are different schema releases; do not adapt the IFC4 XML silently into IFC4x3 enforcement. Direct page fetch was403 in this environment, so no downloaded database hash is claimed.

The [buildingSMART2025 governance announcement](https://www.buildingsmart.org/ifc-4-3-alignment-based-view-definition-1-0-final-project-plan-is-now-is-now-out-for-review-by-the-standards-committee/) describes AbV as a selection of existing concept templates and formal publication process. Current final status may supersede its consultation stage; the announcement is provenance, not the current executable selection manifest. DesignTransfer is not authenticated as a final IFC4x3 executable view here.

Pinned Standards-builds release commit610ee3f95db9d0380b1eb8de4dc6e17833531518 root API lists HTML,IFC4.3.2.0.zip,index.html,lib. HTML immediate listing has concepts but no immediate XML/JSON machine artifact; lib immediate children are mirrored web libraries. This is only exact-directory observation, NOT recursive/ZIP absence proof. Release ZIP contents and complete build-resource tree were not exhausted in this checkpoint. No claim that official machine-readable constraints do not exist anywhere.

## What is executable now and what remains needed

Current schema-first concept evaluator may execute authenticated explicit released clauses (Body Tessellation and Alignment business terminal), with original source capture/index/declared attributes/cancellation and schema/view not-evaluated. Historical mvdXML can provide exact candidate traversal graphs/TemplateRule parameters but must not supply allview applicability or new constraints merely because its XML parses.

Business/geometric correspondence remains as earlier reports: direct terminal product Representation paths are demonstrated by exact official H/V/C models; parent curve lists alone do not establish universal ordinal matching. Historical XML structure should be compared against release diagram and exact clause before selecting a narrow counterpart predicate. General usages are not automatically mandatory ReferenceView requirements.

Required next authoritative evidence: released ADD2 view selection manifest tying exact concept UUIDs/template revisions to ReferenceView/Alignment-basedView; officially published mvdXML or machine schema/hash and interpretation of TemplateRule grouping; release ZIP recursive inventory; build generator source/config pinned to same release commit. If a view is intentionally only a concept selection, freeze the official selection itself separately from concept rule execution and mark unexecutable prose clauses incomplete. AST/catalog compilation, badge tables and this historical sample cannot justify allMVD conformance.
