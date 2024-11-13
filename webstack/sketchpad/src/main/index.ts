import { readFileSync } from 'fs'
import { join } from 'path'
import { app, shell, BrowserWindow, ipcMain, dialog } from 'electron'
import { electronApp, optimizer, is } from '@electron-toolkit/utils'
import { GraphQLClient, gql } from 'graphql-request'
import icon from '../../resources/icon.png?asset'

function createWindow(): void {
    // Create the browser window.
    const mainWindow = new BrowserWindow({
        width: 1080,
        height: 720,
        show: false,
        autoHideMenuBar: true,
        ...(process.platform === 'linux' ? { icon } : {}),
        frame: false,
        webPreferences: {
            preload: join(__dirname, '../preload/index.js'),
            sandbox: false
        }
    })

    // Disable the default menu
    // mainWindow.setMenu(null);

    mainWindow.on('ready-to-show', () => {
        mainWindow.show()
    })

    mainWindow.webContents.setWindowOpenHandler((details) => {
        shell.openExternal(details.url)
        return { action: 'deny' }
    })

    // HMR for renderer base on electron-vite cli.
    // Load the remote URL for development or the local html file for production.
    if (is.dev && process.env['ELECTRON_RENDERER_URL']) {
        mainWindow.loadURL(process.env['ELECTRON_RENDERER_URL'])
    } else {
        mainWindow.loadFile(join(__dirname, '../renderer/index.html'))
    }
}

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app.whenReady().then(() => {
    // Set app user model id for windows
    electronApp.setAppUserModelId('com.electron')

    // Default open or close DevTools by F12 in development
    // and ignore CommandOrControl + R in production.
    // see https://github.com/alex8088/electron-toolkit/tree/master/packages/utils
    app.on('browser-window-created', (_, window) => {
        optimizer.watchWindowShortcuts(window)
    })

    ipcMain.handle('minimize-window', (event) => {
        const window = BrowserWindow.getFocusedWindow()
        if (window) window.minimize()
    })

    ipcMain.handle('maximize-window', (event) => {
        const window = BrowserWindow.getFocusedWindow()
        if (window) {
            if (window.isMaximized()) {
                window.unmaximize()
            } else {
                window.maximize()
            }
        }
    })

    ipcMain.handle('close-window', (event) => {
        const window = BrowserWindow.getFocusedWindow()
        if (window) window.close()
    })

    ipcMain.handle('open-directory', async () => {
        const directory = await dialog.showOpenDialog({
            properties: ['openDirectory']
        })
        if (directory.canceled || directory.filePaths.length === 0) {
            return null
        }
        return directory.filePaths[0]
    })

    ipcMain.handle('open-file', async () => {
        const file = await dialog.showOpenDialog({
            properties: ['openFile']
        })
        if (file.canceled || file.filePaths.length === 0) {
            return null
        }
        return file.filePaths[0]
    })

    const LOAD_LOCAL_KIT = gql`
        query LoadLocalKit($directory: String!) {
            loadLocalKit(directory: $directory) {
                kit {
                    name
                    description
                    icon
                    url
                    homepage
                    types {
                        name
                        description
                        icon
                        variant
                        unit
                        representations {
                            url
                            mime
                            lod
                            tags
                        }
                        ports {
                            id
                            point {
                                x
                                y
                                z
                            }
                            direction {
                                x
                                y
                                z
                            }
                            locators {
                                group
                                subgroup
                            }
                        }
                        qualities {
                            name
                            value
                            unit
                            definition
                        }
                    }
                    designs {
                        name
                        description
                        icon
                        variant
                        unit
                        pieces {
                            id
                            type {
                                name
                                variant
                            }
                            root {
                                plane {
                                    origin {
                                        x
                                        y
                                        z
                                    }
                                    xAxis {
                                        x
                                        y
                                        z
                                    }
                                    yAxis {
                                        x
                                        y
                                        z
                                    }
                                }
                            }
                            diagram {
                                point {
                                    x
                                    y
                                }
                            }
                        }
                        connections {
                            connected {
                                piece {
                                    id
                                    type {
                                        port {
                                            id
                                        }
                                    }
                                }
                            }
                            connecting {
                                piece {
                                    id
                                    type {
                                        port {
                                            id
                                        }
                                    }
                                }
                            }
                            gap
                            rotation
                        }
                    }
                }
                error
            }
        }
    `
    const endpoint = 'http://127.0.0.1:5052/graphql'
    const client = new GraphQLClient(endpoint)
    ipcMain.handle('load-local-kit', async (event, directory) => {
        if (!directory) {
            return
        }
        const response = await client.request(LOAD_LOCAL_KIT, { directory })
        return response
    })

    const ADD_DESIGN_TO_LOCAL_KIT = gql`
        mutation AddDesignToLocalKit($directory: String!, $design: DesignInput!) {
            addDesignToLocalKit(directory: $directory, designInput: $design) {
                design {
                    name
                    description
                    icon
                    variant
                    unit
                    pieces {
                        id
                        type {
                            name
                            variant
                        }
                        root {
                            plane {
                                origin {
                                    x
                                    y
                                    z
                                }
                                xAxis {
                                    x
                                    y
                                    z
                                }
                                yAxis {
                                    x
                                    y
                                    z
                                }
                            }
                        }
                        diagram {
                            point {
                                x
                                y
                            }
                        }
                    }
                    connections {
                        connected {
                            piece {
                                id
                                type {
                                    port {
                                        id
                                    }
                                }
                            }
                        }
                        connecting {
                            piece {
                                id
                                type {
                                    port {
                                        id
                                    }
                                }
                            }
                        }
                        gap
                        rotation
                    }
                }
                error {
                    code
                    message
                }
            }
        }
    `
    ipcMain.handle('add-local-design', async (event, directory, design) => {
        if (!directory) {
            return
        }
        const { addDesignToLocalKit } = await client.request(ADD_DESIGN_TO_LOCAL_KIT, {
            directory,
            design
        })
        return addDesignToLocalKit
    })

    ipcMain.handle('get-file-buffer', async (event, filePath, directory = undefined) => {
        if (directory) {
            filePath = join(directory, filePath)
        }
        return readFileSync(filePath)
    })

    createWindow()

    app.on('activate', function () {
        // On macOS it's common to re-create a window in the app when the
        // dock icon is clicked and there are no other windows open.
        if (BrowserWindow.getAllWindows().length === 0) createWindow()
    })
})

// Quit when all windows are closed, except on macOS. There, it's common
// for applications and their menu bar to stay active until the user quits
// explicitly with Cmd + Q.
app.on('window-all-closed', () => {
    if (process.platform !== 'darwin') {
        app.quit()
    }
})

// In this file you can include the rest of your app"s specific main process
// code. You can also put them in separate files and require them here.
