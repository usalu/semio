import { app, BrowserWindow } from 'electron';
import path from 'node:path';

const createWindow = () => {
    const win = new BrowserWindow({
        width: 800,
        height: 600,
        webPreferences: {
            preload: path.join(__dirname, 'preload.js')
        }
    })

    win.loadURL('http://localhost:5173/');
    // win.loadFile('index.html')
}

app.whenReady().then(() => {
    createWindow()
})