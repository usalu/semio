from google.api import launch_stage_pb2 as _launch_stage_pb2
from google.protobuf import descriptor_pb2 as _descriptor_pb2
from google.protobuf import duration_pb2 as _duration_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

ADS: ClientLibraryOrganization
CLIENT_LIBRARY_DESTINATION_UNSPECIFIED: ClientLibraryDestination
CLIENT_LIBRARY_ORGANIZATION_UNSPECIFIED: ClientLibraryOrganization
CLOUD: ClientLibraryOrganization
DEFAULT_HOST_FIELD_NUMBER: _ClassVar[int]
DESCRIPTOR: _descriptor.FileDescriptor
GITHUB: ClientLibraryDestination
METHOD_SIGNATURE_FIELD_NUMBER: _ClassVar[int]
OAUTH_SCOPES_FIELD_NUMBER: _ClassVar[int]
PACKAGE_MANAGER: ClientLibraryDestination
PHOTOS: ClientLibraryOrganization
STREET_VIEW: ClientLibraryOrganization
default_host: _descriptor.FieldDescriptor
method_signature: _descriptor.FieldDescriptor
oauth_scopes: _descriptor.FieldDescriptor

class ClientLibrarySettings(_message.Message):
    __slots__ = ["cpp_settings", "dotnet_settings", "go_settings", "java_settings", "launch_stage", "node_settings", "php_settings", "python_settings", "rest_numeric_enums", "ruby_settings", "version"]
    CPP_SETTINGS_FIELD_NUMBER: _ClassVar[int]
    DOTNET_SETTINGS_FIELD_NUMBER: _ClassVar[int]
    GO_SETTINGS_FIELD_NUMBER: _ClassVar[int]
    JAVA_SETTINGS_FIELD_NUMBER: _ClassVar[int]
    LAUNCH_STAGE_FIELD_NUMBER: _ClassVar[int]
    NODE_SETTINGS_FIELD_NUMBER: _ClassVar[int]
    PHP_SETTINGS_FIELD_NUMBER: _ClassVar[int]
    PYTHON_SETTINGS_FIELD_NUMBER: _ClassVar[int]
    REST_NUMERIC_ENUMS_FIELD_NUMBER: _ClassVar[int]
    RUBY_SETTINGS_FIELD_NUMBER: _ClassVar[int]
    VERSION_FIELD_NUMBER: _ClassVar[int]
    cpp_settings: CppSettings
    dotnet_settings: DotnetSettings
    go_settings: GoSettings
    java_settings: JavaSettings
    launch_stage: _launch_stage_pb2.LaunchStage
    node_settings: NodeSettings
    php_settings: PhpSettings
    python_settings: PythonSettings
    rest_numeric_enums: bool
    ruby_settings: RubySettings
    version: str
    def __init__(self, version: _Optional[str] = ..., launch_stage: _Optional[_Union[_launch_stage_pb2.LaunchStage, str]] = ..., rest_numeric_enums: bool = ..., java_settings: _Optional[_Union[JavaSettings, _Mapping]] = ..., cpp_settings: _Optional[_Union[CppSettings, _Mapping]] = ..., php_settings: _Optional[_Union[PhpSettings, _Mapping]] = ..., python_settings: _Optional[_Union[PythonSettings, _Mapping]] = ..., node_settings: _Optional[_Union[NodeSettings, _Mapping]] = ..., dotnet_settings: _Optional[_Union[DotnetSettings, _Mapping]] = ..., ruby_settings: _Optional[_Union[RubySettings, _Mapping]] = ..., go_settings: _Optional[_Union[GoSettings, _Mapping]] = ...) -> None: ...

class CommonLanguageSettings(_message.Message):
    __slots__ = ["destinations", "reference_docs_uri"]
    DESTINATIONS_FIELD_NUMBER: _ClassVar[int]
    REFERENCE_DOCS_URI_FIELD_NUMBER: _ClassVar[int]
    destinations: _containers.RepeatedScalarFieldContainer[ClientLibraryDestination]
    reference_docs_uri: str
    def __init__(self, reference_docs_uri: _Optional[str] = ..., destinations: _Optional[_Iterable[_Union[ClientLibraryDestination, str]]] = ...) -> None: ...

class CppSettings(_message.Message):
    __slots__ = ["common"]
    COMMON_FIELD_NUMBER: _ClassVar[int]
    common: CommonLanguageSettings
    def __init__(self, common: _Optional[_Union[CommonLanguageSettings, _Mapping]] = ...) -> None: ...

class DotnetSettings(_message.Message):
    __slots__ = ["common"]
    COMMON_FIELD_NUMBER: _ClassVar[int]
    common: CommonLanguageSettings
    def __init__(self, common: _Optional[_Union[CommonLanguageSettings, _Mapping]] = ...) -> None: ...

class GoSettings(_message.Message):
    __slots__ = ["common"]
    COMMON_FIELD_NUMBER: _ClassVar[int]
    common: CommonLanguageSettings
    def __init__(self, common: _Optional[_Union[CommonLanguageSettings, _Mapping]] = ...) -> None: ...

class JavaSettings(_message.Message):
    __slots__ = ["common", "library_package", "service_class_names"]
    class ServiceClassNamesEntry(_message.Message):
        __slots__ = ["key", "value"]
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    COMMON_FIELD_NUMBER: _ClassVar[int]
    LIBRARY_PACKAGE_FIELD_NUMBER: _ClassVar[int]
    SERVICE_CLASS_NAMES_FIELD_NUMBER: _ClassVar[int]
    common: CommonLanguageSettings
    library_package: str
    service_class_names: _containers.ScalarMap[str, str]
    def __init__(self, library_package: _Optional[str] = ..., service_class_names: _Optional[_Mapping[str, str]] = ..., common: _Optional[_Union[CommonLanguageSettings, _Mapping]] = ...) -> None: ...

class MethodSettings(_message.Message):
    __slots__ = ["long_running", "selector"]
    class LongRunning(_message.Message):
        __slots__ = ["initial_poll_delay", "max_poll_delay", "poll_delay_multiplier", "total_poll_timeout"]
        INITIAL_POLL_DELAY_FIELD_NUMBER: _ClassVar[int]
        MAX_POLL_DELAY_FIELD_NUMBER: _ClassVar[int]
        POLL_DELAY_MULTIPLIER_FIELD_NUMBER: _ClassVar[int]
        TOTAL_POLL_TIMEOUT_FIELD_NUMBER: _ClassVar[int]
        initial_poll_delay: _duration_pb2.Duration
        max_poll_delay: _duration_pb2.Duration
        poll_delay_multiplier: float
        total_poll_timeout: _duration_pb2.Duration
        def __init__(self, initial_poll_delay: _Optional[_Union[_duration_pb2.Duration, _Mapping]] = ..., poll_delay_multiplier: _Optional[float] = ..., max_poll_delay: _Optional[_Union[_duration_pb2.Duration, _Mapping]] = ..., total_poll_timeout: _Optional[_Union[_duration_pb2.Duration, _Mapping]] = ...) -> None: ...
    LONG_RUNNING_FIELD_NUMBER: _ClassVar[int]
    SELECTOR_FIELD_NUMBER: _ClassVar[int]
    long_running: MethodSettings.LongRunning
    selector: str
    def __init__(self, selector: _Optional[str] = ..., long_running: _Optional[_Union[MethodSettings.LongRunning, _Mapping]] = ...) -> None: ...

class NodeSettings(_message.Message):
    __slots__ = ["common"]
    COMMON_FIELD_NUMBER: _ClassVar[int]
    common: CommonLanguageSettings
    def __init__(self, common: _Optional[_Union[CommonLanguageSettings, _Mapping]] = ...) -> None: ...

class PhpSettings(_message.Message):
    __slots__ = ["common"]
    COMMON_FIELD_NUMBER: _ClassVar[int]
    common: CommonLanguageSettings
    def __init__(self, common: _Optional[_Union[CommonLanguageSettings, _Mapping]] = ...) -> None: ...

class Publishing(_message.Message):
    __slots__ = ["api_short_name", "codeowner_github_teams", "doc_tag_prefix", "documentation_uri", "github_label", "library_settings", "method_settings", "new_issue_uri", "organization"]
    API_SHORT_NAME_FIELD_NUMBER: _ClassVar[int]
    CODEOWNER_GITHUB_TEAMS_FIELD_NUMBER: _ClassVar[int]
    DOCUMENTATION_URI_FIELD_NUMBER: _ClassVar[int]
    DOC_TAG_PREFIX_FIELD_NUMBER: _ClassVar[int]
    GITHUB_LABEL_FIELD_NUMBER: _ClassVar[int]
    LIBRARY_SETTINGS_FIELD_NUMBER: _ClassVar[int]
    METHOD_SETTINGS_FIELD_NUMBER: _ClassVar[int]
    NEW_ISSUE_URI_FIELD_NUMBER: _ClassVar[int]
    ORGANIZATION_FIELD_NUMBER: _ClassVar[int]
    api_short_name: str
    codeowner_github_teams: _containers.RepeatedScalarFieldContainer[str]
    doc_tag_prefix: str
    documentation_uri: str
    github_label: str
    library_settings: _containers.RepeatedCompositeFieldContainer[ClientLibrarySettings]
    method_settings: _containers.RepeatedCompositeFieldContainer[MethodSettings]
    new_issue_uri: str
    organization: ClientLibraryOrganization
    def __init__(self, method_settings: _Optional[_Iterable[_Union[MethodSettings, _Mapping]]] = ..., new_issue_uri: _Optional[str] = ..., documentation_uri: _Optional[str] = ..., api_short_name: _Optional[str] = ..., github_label: _Optional[str] = ..., codeowner_github_teams: _Optional[_Iterable[str]] = ..., doc_tag_prefix: _Optional[str] = ..., organization: _Optional[_Union[ClientLibraryOrganization, str]] = ..., library_settings: _Optional[_Iterable[_Union[ClientLibrarySettings, _Mapping]]] = ...) -> None: ...

class PythonSettings(_message.Message):
    __slots__ = ["common"]
    COMMON_FIELD_NUMBER: _ClassVar[int]
    common: CommonLanguageSettings
    def __init__(self, common: _Optional[_Union[CommonLanguageSettings, _Mapping]] = ...) -> None: ...

class RubySettings(_message.Message):
    __slots__ = ["common"]
    COMMON_FIELD_NUMBER: _ClassVar[int]
    common: CommonLanguageSettings
    def __init__(self, common: _Optional[_Union[CommonLanguageSettings, _Mapping]] = ...) -> None: ...

class ClientLibraryOrganization(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []

class ClientLibraryDestination(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []
