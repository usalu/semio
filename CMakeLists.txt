cmake_minimum_required(VERSION 3.28)

project(ComposeNative VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23 CACHE STRING "C++ language standard")
set(CMAKE_CXX_STANDARD_REQUIRED ON CACHE BOOL "Require the configured C++ standard")
set(CMAKE_CXX_EXTENSIONS OFF CACHE BOOL "Disable compiler-specific C++ extensions")
set(CMAKE_EXPORT_COMPILE_COMMANDS ON CACHE BOOL "Export compile_commands.json")
set(CMAKE_POSITION_INDEPENDENT_CODE ON CACHE BOOL "Build position-independent native code")

option(COMPOSE_CPP_TOPOLOGIC "Build the Topologic C++ geometry package" ON)
option(COMPOSE_CPP_ENABLE_TESTS "Enable native CTest registration" ON)

if(COMPOSE_CPP_ENABLE_TESTS)
  include(CTest)
endif()

if(MSVC)
  add_compile_options(/permissive- /Zc:__cplusplus /W4)
else()
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

if(COMPOSE_CPP_TOPOLOGIC)
  add_subdirectory(elements/client/lib/geometry/topologic)
endif()
