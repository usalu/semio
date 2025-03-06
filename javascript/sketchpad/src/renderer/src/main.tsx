import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import { Provider } from 'react-redux'
import { store } from './store'


const onWindowMinimize = (): void => {
    window.electron.ipcRenderer.invoke('minimize-window')
}

const onWindowMaximize = (): void => {
    window.electron.ipcRenderer.invoke('maximize-window')
}

const onWindowClose = (): void => {
    window.electron.ipcRenderer.invoke('close-window')
}

const onOpenDirectory = (): Promise<string> => {
    return window.electron.ipcRenderer.invoke('open-directory')
}

const onOpenFile = (): Promise<string> => {
    return window.electron.ipcRenderer.invoke('open-file')
}

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <Provider store={store}>
            <App
                onWindowMinimize={onWindowMinimize}
                onWindowMaximize={onWindowMaximize}
                onWindowClose={onWindowClose}
                onOpenDirectory={onOpenDirectory}
                onOpenFile={onOpenFile}
            />
        </Provider>
    </React.StrictMode>
)