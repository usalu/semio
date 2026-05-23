$path = "c:\git\semio\semio\client\lib\sketchpad\react\index.tsx"
$content = Get-Content $path -Raw
$start = $content.IndexOf("        <LevelProvider level=`"base`">`r`n          <ToolbarContextHost>`r`n            <LayoutComponent")
if ($start -lt 0) { $start = $content.IndexOf("        <LevelProvider level=`"base`">`n          <ToolbarContextHost>`n            <LayoutComponent") }
$endMarker = "            />`r`n          </ToolbarContextHost>`r`n        </LevelProvider>"
$end = $content.IndexOf($endMarker, $start)
if ($end -lt 0) {
  $endMarker = "            />`n          </ToolbarContextHost>`n        </LevelProvider>"
  $end = $content.IndexOf($endMarker, $start)
}
if ($start -lt 0 -or $end -lt 0) { throw "markers not found start=$start end=$end" }
$replacement = @'
        <LevelProvider level="base">
          <ToolbarContextHost>
            <SketchpadDeclarativeWorkbenchHost
              toolbarSlot={
                panelVisibility.toolbar || appType === "type" || appType === "design" || appType === "feedback" || appType === "kit" || appType === "home" || appType === "docs" ? (
                  toolbarSections.length > 0 ? (
                    <div role="toolbar" id="semio.sketchpad.toolbar" className="pointer-events-none absolute bottom-1.5 left-0 right-0 h-[40px] w-full max-w-full px-2">
                      <div id="semio.sketchpad.toolbar.seam" className="absolute left-1/2 top-0 h-full w-0 -translate-x-1/2 pointer-events-none" aria-hidden />
                      <div ref={toolbarToolsZoneRef} id="semio.sketchpad.toolbar.zone.tools" className="absolute top-0 left-0 h-full max-w-[calc(50vw-1rem)] right-[calc(50%_+_4px)] pointer-events-auto flex items-center justify-end">
                        <LevelProvider level="panel">
                          <ToolbarZone>
                            {toolbarGroups.history && (
                              <ToolbarContextHost>
                                {toolbarGroups.history.map((section) => {
                                  return <ToolbarItem key={section.id}>{typeof section.content === "function" ? section.content() : section.content}</ToolbarItem>;
                                })}
                              </ToolbarContextHost>
                            )}
                            {orderedToolbarGroupIds
                              .filter((groupId) => groupId !== "history")
                              .map((groupId) => {
                                if (!toolbarGroups[groupId]) return null;
                                const isActive = activeToolbarGroup === groupId;

                                return (
                                  <Toggle
                                    key={groupId}
                                    kind="single"
                                    id={`semio.sketchpad.toolbar.group.${groupId}`}
                                    pressed={isActive}
                                    onPressedChange={() => toggleToolbarGroup(groupId)}
                                    icon={getGroupIcon(groupId)}
                                    text={resolveTranslationLabel(i18n.t(`semio.sketchpad.toolbar.parent.${groupId}`))}
                                  />
                                );
                              })}
                          </ToolbarZone>
                        </LevelProvider>
                      </div>

                      {activeToolbarGroup && toolbarGroups[activeToolbarGroup] && (
                        <div ref={toolbarSettingsZoneRef} id="semio.sketchpad.toolbar.zone.settings" className="absolute top-0 left-[calc(50%_+_4px)] right-0 h-full min-w-0 pointer-events-auto flex items-center justify-start">
                          <LevelProvider level="panel">
                            <div ref={toolbarSettingsContentRef} className="flex w-fit max-w-full shrink-0">
                              <ToolbarZone className="w-fit max-w-full shrink-0 flex-nowrap">
                                <ToolbarContextHost>
                                  {toolbarGroups[activeToolbarGroup]?.map((section) => {
                                    return <ToolbarItem key={section.id}>{typeof section.content === "function" ? section.content() : section.content}</ToolbarItem>;
                                  })}
                                </ToolbarContextHost>
                              </ToolbarZone>
                            </div>
                          </LevelProvider>
                        </div>
                      )}
                    </div>
                  ) : (
                    <div id="semio.sketchpad.toolbar" className="hidden" />
                  )
                ) : undefined
              }
              initialPanelVisibility={{
                leftSidePanel: Boolean(panelVisibility.leftSidePanel),
                rightSidePanel: Boolean(panelVisibility.rightSidePanel || panelVisibility.details || panelVisibility.chat || panelVisibility.settings),
              }}
              navigationUri={currentPath}
              canGoBack={navigationHistory.canGoBack}
              canGoForward={navigationHistory.canGoForward}
              canGoUp={Boolean(upTarget)}
              onNavigate={(uri) => reactNavigate(uri)}
              onGoBack={() => sketchpadCommands.navigateBack("semio.sketchpad.navbar.back")}
              onGoForward={() => sketchpadCommands.navigateForward("semio.sketchpad.navbar.forward")}
              onGoUp={() => {
                if (upTarget) reactNavigate(upTarget);
              }}
            />
          </ToolbarContextHost>
        </LevelProvider>
'@
$newContent = $content.Substring(0, $start) + $replacement + $content.Substring($end + $endMarker.Length)
Set-Content $path $newContent -NoNewline -Encoding utf8
Write-Host "replaced layout block"
