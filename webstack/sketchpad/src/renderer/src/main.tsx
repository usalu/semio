import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import { IDraft } from './App'
import { Kit } from './semio'

const onWindowMinimize = (): void => {
    window.electron.ipcRenderer.invoke('minimize-window')
}

const onWindowMaximize = (): void => {
    window.electron.ipcRenderer.invoke('maximize-window')
}

const onWindowClose = (): void => {
    window.electron.ipcRenderer.invoke('close-window')
}

const onOpenKit = (): Promise<Kit> => {
    return window.electron.ipcRenderer.invoke('open-kit')
}

const onReloadKit = (): Promise<Kit> => {
    return window.electron.ipcRenderer.invoke('reload-kit')
}

const onOpenDraft = (): Promise<string> => {
    return window.electron.ipcRenderer.invoke('open-draft')
}

const onSaveDraft = (draft: IDraft): Promise<string> => {
    return window.electron.ipcRenderer.invoke('save-draft', JSON.stringify(draft))
}

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <App
            onWindowMinimize={onWindowMinimize}
            onWindowMaximize={onWindowMaximize}
            onWindowClose={onWindowClose}
            onOpenKit={onOpenKit}
            onReloadKit={onReloadKit}
            onOpenDraft={onOpenDraft}
            onSaveDraft={onSaveDraft}
        />
    </React.StrictMode>
)
