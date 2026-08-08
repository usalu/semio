#!/usr/bin/env python3
from __future__ import annotations
from pathlib import Path
import json
PLUGIN = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect")
ART = PLUGIN / "🗿️artifacts" / "🏛️program"
TICKET = Path('/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️08/ARTIFACT-SCHEMA-FACETS')
exec((TICKET / "🧪wave5-architect-helpers-core.py").read_text(), globals())
COLLECTIONS_META = [
  ('stakeholders', 'Stakeholder'),
  ('users', 'UserProfile'),
  ('activities', 'Activity'),
  ('functions', 'Function'),
  ('elements', 'ProgramElement'),
  ('quantities', 'QuantityRequirement'),
  ('relationships', 'Relationship'),
  ('adjacencies', 'Adjacency'),
  ('processes', 'Process'),
  ('flows', 'FlowRequirement'),
  ('access_rules', 'AccessRule'),
  ('operations', 'OperationalRequirement'),
  ('equipment', 'Equipment'),
  ('resources', 'Resource'),
  ('storage', 'StorageRequirement'),
  ('environmental', 'EnvironmentalRequirement'),
  ('human_factors', 'HumanFactorRequirement'),
  ('accessibility', 'AccessibilityRequirement'),
  ('privacy', 'PrivacyRequirement'),
  ('safety', 'SafetyRequirement'),
  ('security', 'SecurityRequirement'),
  ('regulatory', 'RegulatoryRequirement'),
  ('site_context', 'SiteContext'),
  ('organizational', 'OrganizationalRequirement'),
  ('services', 'ServiceRequirement'),
  ('infrastructure', 'InfrastructureRequirement'),
  ('information', 'InformationRequirement'),
  ('communication', 'CommunicationRequirement'),
  ('wayfinding', 'WayfindingRequirement'),
  ('schedules', 'ScheduleRequirement'),
  ('flexibility', 'FlexibilityRequirement'),
  ('growth', 'GrowthPlan'),
  ('sustainability', 'SustainabilityRequirement'),
  ('resilience', 'ResilienceRequirement'),
  ('costs', 'CostRequirement'),
  ('delivery', 'DeliveryConstraint'),
  ('risks', 'Risk'),
  ('conflicts', 'Conflict'),
  ('requirements', 'Requirement'),
  ('priorities', 'PriorityRecord'),
  ('scenarios', 'Scenario'),
  ('options', 'OptionEvaluation'),
  ('decisions', 'Decision'),
  ('validations', 'ValidationRecord'),
  ('performance', 'PerformanceCriterion'),
  ('quality', 'QualityRecord'),
  ('documents', 'DocumentRecord'),
  ('assumptions', 'Assumption'),
  ('constraints', 'ConstraintRecord'),
  ('compliance_records', 'ComplianceRecord'),
  ('approvals', 'ApprovalRecord'),
  ('meetings', 'MeetingRecord'),
  ('changes', 'ChangeRecord'),
  ('collaboration', 'CollaborationRecord'),
  ('analyses', 'AnalysisRecord'),
  ('reports', 'ReportRecord'),
  ('search_filters', 'SearchFilter'),
  ('status_records', 'StatusRecord'),
  ('workshops', 'Workshop'),
  ('surveys', 'Survey'),
  ('issues', 'Issue'),
  ('audit_events', 'AuditEvent'),
  ('templates', 'TemplateRecord'),
  ('knowledge', 'KnowledgeRecord'),
  ('benchmarks', 'BenchmarkRecord'),
  ('traces', 'TraceLink'),
]

def camel(s):
    p = s.split('_'); return p[0] + ''.join(x.title() for x in p[1:])
def pascal(s):
    return ''.join(x.title() for x in s.split('_'))

PERSISTENT = [
    ('schema','schema','String','persistent',False,'scalar','string'),
    ('meta','meta','ProgramMeta','persistent',False,'scalar','ProgramMeta'),
    ('project','project','ProjectDefinition','persistent',False,'scalar','ProjectDefinition'),
]
for snake, item in COLLECTIONS_META:
    PERSISTENT.append((camel(snake), snake, f'Vec<{item}>', 'persistent', False, 'list', item))
PERSISTENT.append(('governance','governance','Governance','persistent',False,'scalar','Governance'))

UI = [
    ('selectedIds','selected_ids','Vec<String>','shared_ui',False,'list','string'),
    ('activeRegister','active_register','String','shared_ui',False,'scalar','string'),
    ('adjacencyKindFilter','adjacency_kind_filter','Option<AdjacencyKind>','shared_ui',True,'scalar','AdjacencyKind'),
    ('activeReportJson','active_report_json','String','shared_ui',False,'scalar','string'),
    ('searchQuery','search_query','String','local_ui',False,'scalar','string'),
    ('searchHistoryJson','search_history_json','String','local_ui',False,'scalar','string'),
    ('lastResultJson','last_result_json','String','local_ui',False,'scalar','string'),
    ('lastAnalysisJson','last_analysis_json','String','local_ui',False,'scalar','string'),
    ('graphCameraX','graph_camera_x','f64','local_ui',False,'scalar','float64'),
    ('graphCameraY','graph_camera_y','f64','local_ui',False,'scalar','float64'),
    ('graphCameraZoom','graph_camera_zoom','f64','local_ui',False,'scalar','float64'),
]
COLLECTION_CAMELS = [camel(s) for s,_ in COLLECTIONS_META]
DELTA_ITEM = {camel(s): t for s,t in COLLECTIONS_META}
NESTED_TYPES = sorted({t for _,t in COLLECTIONS_META} | {'ProgramMeta','ProjectDefinition','Governance','AdjacencyKind'})
DEFS = {t: {'title': t, 'type': 'object', 'additionalProperties': True} for t in NESTED_TYPES}
print('READY', len(PERSISTENT), len(UI), COLLECTIONS_META[31])

exec((TICKET / "🧪wave5-architect-emit.py").read_text(), globals())

exec((TICKET / "🧪wave5-architect-emit-rest.py").read_text(), globals())
