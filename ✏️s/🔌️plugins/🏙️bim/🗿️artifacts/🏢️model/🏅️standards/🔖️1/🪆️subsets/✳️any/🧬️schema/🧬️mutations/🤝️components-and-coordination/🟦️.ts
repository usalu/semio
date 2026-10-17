/** 🤝️ BIM model direct mutations for components, MEP elements, families, clash sets, rules, issues, property templates and classification systems. */
import type { ClashRef, ClashSet, ClassificationItem, ClassificationSystem, Component, ElementSelector, ExprPoint3, Family, FamilyCategory, FamilySolid, Issue, IssueComment, IssuePriority, IssueStatus, IssueViewpoint, MepElement, MepShape, MepSystem, ParameterKind, Point2, Point3, PropertyDef, PropertyTemplate, Rule, RuleKind, RuleScope, RuleSeverity, SolidShape, TemplateTarget } from "../../🟦️.ts";

export interface CreateComponent {
  mutation: "createComponent";
  id: string;
  component: Component;
}

export interface SetComponent {
  mutation: "setComponent";
  id: string;
  storey?: string;
  family?: string;
  position?: Point2;
  elevation?: number;
  rotation?: number;
  mirrored?: boolean;
  host?: Record<string, unknown>;
  system?: MepSystem;
  name?: string;
}

export interface DeleteComponent {
  mutation: "deleteComponent";
  id: string;
}

export interface SetComponentOverride {
  mutation: "setComponentOverride";
  component: string;
  name: string;
  value: string;
}

export interface RemoveComponentOverride {
  mutation: "removeComponentOverride";
  component: string;
  name: string;
}

export interface CreateMepElement {
  mutation: "createMepElement";
  id: string;
  mep: MepElement;
}

export interface SetMepElement {
  mutation: "setMepElement";
  id: string;
  storey?: string;
  system?: MepSystem;
  shape?: MepShape;
  path?: Point3[];
  name?: string;
}

export interface DeleteMepElement {
  mutation: "deleteMepElement";
  id: string;
}

export interface CreateClashSet {
  mutation: "createClashSet";
  id: string;
  clash_set: ClashSet;
}

export interface SetClashSet {
  mutation: "setClashSet";
  id: string;
  name?: string;
  a?: ElementSelector;
  b?: ElementSelector;
  tolerance?: number;
  clearance?: number;
}

export interface DeleteClashSet {
  mutation: "deleteClashSet";
  id: string;
}

export interface CreateRule {
  mutation: "createRule";
  id: string;
  rule: Rule;
}

export interface SetRule {
  mutation: "setRule";
  id: string;
  name?: string;
  kind?: RuleKind;
  limit?: number;
  severity?: RuleSeverity;
  scope?: RuleScope;
}

export interface DeleteRule {
  mutation: "deleteRule";
  id: string;
}

export interface CreateIssue {
  mutation: "createIssue";
  id: string;
  issue: Issue;
}

export interface SetIssue {
  mutation: "setIssue";
  id: string;
  title?: string;
  description?: string;
  status?: IssueStatus;
  priority?: IssuePriority;
  assignee?: string;
  author?: string;
  created?: string;
  labels?: string[];
  elements?: string[];
  clash?: ClashRef;
  viewpoint?: IssueViewpoint;
}

export interface DeleteIssue {
  mutation: "deleteIssue";
  id: string;
}

export interface CreateIssueComment {
  mutation: "createIssueComment";
  id: string;
  issue_comment: IssueComment;
}

export interface SetIssueComment {
  mutation: "setIssueComment";
  id: string;
  author?: string;
  date?: string;
  text?: string;
}

export interface DeleteIssueComment {
  mutation: "deleteIssueComment";
  id: string;
}

export interface CreatePropertyTemplate {
  mutation: "createPropertyTemplate";
  id: string;
  template: PropertyTemplate;
}

export interface SetPropertyTemplate {
  mutation: "setPropertyTemplate";
  id: string;
  name?: string;
  applies_to?: TemplateTarget[];
  properties?: PropertyDef[];
}

export interface DeletePropertyTemplate {
  mutation: "deletePropertyTemplate";
  id: string;
}

export interface CreateClassificationSystem {
  mutation: "createClassificationSystem";
  id: string;
  system: ClassificationSystem;
}

export interface SetClassificationSystem {
  mutation: "setClassificationSystem";
  id: string;
  name?: string;
  edition?: string;
  source?: Record<string, unknown>;
  entries?: ClassificationItem[];
}

export interface DeleteClassificationSystem {
  mutation: "deleteClassificationSystem";
  id: string;
}

export interface CreateFamily {
  mutation: "createFamily";
  id: string;
  family: Family;
}

export interface DeleteFamily {
  mutation: "deleteFamily";
  id: string;
}

export interface SetFamily {
  mutation: "setFamily";
  id: string;
  name?: string;
  category?: FamilyCategory;
}

export interface SetFamilyParameter {
  mutation: "setFamilyParameter";
  family: string;
  name: string;
  kind?: ParameterKind;
  value?: string;
}

export interface RemoveFamilyParameter {
  mutation: "removeFamilyParameter";
  family: string;
  name: string;
}

export interface CreateFamilySolid {
  mutation: "createFamilySolid";
  id: string;
  solid: FamilySolid;
}

export interface DeleteFamilySolid {
  mutation: "deleteFamilySolid";
  id: string;
}

export interface SetFamilySolid {
  mutation: "setFamilySolid";
  id: string;
  name?: string;
  shape?: SolidShape;
  material?: string;
  visible?: string;
  offset?: ExprPoint3;
}
